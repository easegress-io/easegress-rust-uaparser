use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct Device<'a> {
    pub device: Option<Cow<'a, str>>,
}
