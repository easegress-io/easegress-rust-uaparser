use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct OS<'a> {
    pub os: Option<Cow<'a, str>>,
}
