use crate::ui::widget::{canvas::Canvas, container::Container, Widget};
use std::rc::Rc;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct PreRenderer;
impl PreRenderer {
    pub(crate) fn new() -> Self {
        Self {}
    }
    /// Adjust text layout of widgets based on
    /// user settings
    fn adjust_text_layout(&self, widget: &Rc<dyn Widget>) {
        let mut widget_base = widget.base_mut();

        if !widget_base.text.label.is_empty() {
            // Center text horizontally
            if widget_base.text.halign {
                let new_x = widget_base
                    .layout
                    .horizontal_center(widget_base.text.get_true_dimensions().x);
                widget_base.text.pos.x = new_x;
            }
            // Center text vertically
            if widget_base.text.valign {
                let new_y = widget_base
                    .layout
                    .vertical_center(widget_base.text.get_true_dimensions().y);
                widget_base.text.pos.y = new_y;
            }
            // Auto-inherit layout if no specfied
            if widget_base.layout.w == 0.0 {
                widget_base.layout.w = widget_base.text.get_true_dimensions().x
            }
            if widget_base.layout.h == 0.0 {
                widget_base.layout.h = widget_base.text.get_true_dimensions().y
            }
        }
    }
    /// Adjust layout of widgets based on
    /// user settings
    fn adjust_layout(&self, widget: &Container) {
        // Adjust spacing layout
        match widget.flex {
            crate::ui::layout::FlexLayout::None => widget.create_normal_layout(),
            crate::ui::layout::FlexLayout::Col => widget.create_flex_col_layout(),
            crate::ui::layout::FlexLayout::Grid(cols) => widget.create_flex_grid_layout(cols),
        }
    }
    /// Adjust scrollbars
    fn adjust_scrolling(&self, widget: &Container) {
        if let Some(scrollbar) = &widget.scrollbar {
            let (x, y) = scrollbar;
            let widget_base = widget.base();

            let mut x_base = x.base_mut();
            x_base.layout.y = (widget_base.layout.h + widget_base.layout.y) - x_base.layout.h;
            if x_base.layout.x == 0.0 {
                x_base.layout.x = widget_base.layout.x;
            }
            // Create scrollbar to be balanced based on max amount of overflow
            // occuring..otherwise its not seen if no overflow occurs
            let container_width = widget_base.layout.w + widget_base.layout.x;
            let overflow_x = widget
                .children
                .iter()
                .fold(container_width, |acc, child| child.base().layout.w.max(acc));
            let amount_to_take = container_width / overflow_x;
            // Basically makes x scrollbar visible
            if amount_to_take < 1.0 {
                x_base.layout.w = amount_to_take * widget_base.layout.w;
            }

            let mut y_base = y.base_mut();
            y_base.layout.x = (widget_base.layout.w + widget_base.layout.x) - y_base.layout.w;

            // This check prevents the scrollbar from being stucked
            // when redraws occur
            if y_base.layout.y == 0.0 {
                y_base.layout.y = widget_base.layout.y;
            }
            // Create scrollbar to be balanced based on max amount of overflow
            // occuring..otherwise its not seen if no overflow occurs
            let container_height = widget_base.layout.h + widget_base.layout.y;
            let last_child = &widget.children[widget.children.len() - 1];
            let last_child_base = last_child.base();
            let overflow_y = last_child_base.layout.y + last_child_base.layout.h;
            let amount_to_take = container_height / overflow_y;
            // Basically makes y scrollbar visible
            if amount_to_take < 1.0 {
                y_base.layout.h = amount_to_take * widget_base.layout.h;
            }
        }
    }
    /// Make all adjustments
    /// that must propagate first
    fn adjust_children(&self, widget: &Rc<dyn Widget>) {
        self.adjust_text_layout(widget);

        if let Some(widget) = widget.as_any().downcast_ref::<Container>() {
            // Propagate changes down to children
            for child in &widget.children {
                self.adjust_children(child);
            }
        }
    }
    /// Make all adjustments for widgets that do NOT
    /// need to be propagated to children
    ///
    /// Some actions user selects could trigger mutation
    /// of surrounding widgets or attributes
    pub(crate) fn adjust(&self, widget: &Rc<dyn Widget>) {
        self.adjust_children(widget);

        if let Some(widget) = widget.as_any().downcast_ref::<Container>() {
            self.adjust_layout(widget);
            self.adjust_scrolling(widget);

            // Propagate changes down to children
            for child in &widget.children {
                self.adjust(child);
            }
        } else if let Some(widget) = widget.as_any().downcast_ref::<Canvas>() {
            if let Some(grid) = &mut *widget.grid.borrow_mut() {
                let widget_base = widget.base();
                grid.resize(
                    widget_base.layout.x,
                    widget_base.layout.y,
                    widget_base.layout.h,
                    widget_base.layout.w,
                );
            }
        }
    }
}
