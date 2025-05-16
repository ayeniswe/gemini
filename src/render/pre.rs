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
            if widget_base.layout.w == 0 {
                widget_base.layout.w = widget_base.text.get_true_dimensions().x
            }
            if widget_base.layout.h == 0 {
                widget_base.layout.h = widget_base.text.get_true_dimensions().y
            }
        }
    }
    /// Adjust layout of widgets based on
    /// user settings
    fn adjust_layout(&self, widget: &Rc<dyn Widget>) {
        if let Some(widget) = widget.as_any().downcast_ref::<Container>() {
            // Adjust spacing layout
            match widget.flex {
                crate::ui::layout::FlexLayout::None => widget.create_normal_layout(),
                crate::ui::layout::FlexLayout::Col => widget.create_flex_col_layout(),
                crate::ui::layout::FlexLayout::Grid(cols) => widget.create_flex_grid_layout(cols),
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
        
        self.adjust_layout(widget);
        if let Some(widget) = widget.as_any().downcast_ref::<Container>() {
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
