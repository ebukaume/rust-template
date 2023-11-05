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
