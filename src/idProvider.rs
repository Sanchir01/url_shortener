pub trait IdProvider {
    fn provide(&self) -> String;
}

pub struct FakeIdProvider {
    id: String,
}
impl FakeIdProvider {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
impl IdProvider for FakeIdProvider {
    fn provide(&self) -> String {
        self.id.clone()
    }
}
