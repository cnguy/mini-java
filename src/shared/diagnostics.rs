pub struct Diagnostics {
    pub errors: Vec<&'static str>, // static str to keep it simple
}

impl Diagnostics {
    pub fn make() -> Diagnostics {
        Diagnostics { errors: Vec::new() }
    }

    pub fn report(&mut self, error: &'static str) {
        self.errors.push(error);
    }

    pub fn has_no_errors(&self) -> bool {
        self.errors.is_empty()
    }
}

