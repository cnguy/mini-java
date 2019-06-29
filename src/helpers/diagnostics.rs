pub struct Diagnostics {
    errors: Vec<&'static str>, // static str to keep it simple
};

impl Diagnostics {
    pub fn has_no_errors(&self) -> bool {
        return self.errors.is_empty();
    }
}