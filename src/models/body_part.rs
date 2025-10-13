#[derive(Clone, PartialEq)]
pub struct BodyPart {
    pub name: &'static str,
    pub exercises: &'static [&'static str],
}