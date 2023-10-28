use std::borrow::Cow;

pub struct Constant<'a> {
    pub app_name: Cow<'a, str>,
}

impl Constant<'_> {
    pub fn new() -> Self {
        Self {
            app_name: "APPLICATION NAME GOES HERE".into(),
        }
    }
}
