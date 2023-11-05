pub struct Constant {
    pub app_name: String,
}

impl Constant {
    pub fn new() -> Self {
        Self {
            app_name: "APPLICATION NAME GOES HERE".into(),
        }
    }
}

impl Default for Constant {
    fn default() -> Self {
        Self::new()
    }
}
