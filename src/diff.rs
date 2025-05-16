pub (crate) fn diff(&self, rhs: &HashMap<String,System>) -> (Vec<String>,Vec<String>, Vec<String>) {
    let missing = Vec::new();
    let new = Vec::new();
    let mismatch = Vec::new();

    // Check for missing and mismatches
    for (key, lhs_val) in self.software {
        match rhs.get(key) {
            Some(rhs_val) => {
                if lhs_val != rhs_val {
                    mismatch.push(key.clone());
                }
            }
            None => missing.push(key.clone()),
        }
    }

    // Check for new entries
    for key in rhs.keys() {
        if !self.software.contains_key(key) {
            new.push(key.clone());
        }
    }

    (missing, new, mismatch)
}