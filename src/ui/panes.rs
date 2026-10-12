pub enum PaneState {
    Fetch,
    Data,
}

pub struct UiPane {
    pub id: usize,
}
impl UiPane {
    pub fn new(id: usize) -> Self {
        UiPane { id }
    }
}
