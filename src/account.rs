/// Represents a single account
#[derive(Debug, PartialEq)]
pub struct Account {
    pub available: f32,
    pub held: f32,
    pub locked: bool,
}

impl Account {
    pub(crate) fn new(available: f32, held: f32) -> Self {
        Self {
            available,
            held,
            locked: false,
        }
    }
}
