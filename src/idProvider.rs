pub trait IdProvider {
    fn provide(&self) -> String;
}

pub struct NanoIdProvider;
pub struct FakeIdProvider {
    id: String,
}

impl FakeIdProvider {
    pub fn new(id: String) -> Self {
        Self { id }
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id
    }
}
impl IdProvider for FakeIdProvider {
    fn provide(&self) -> String {
        self.id.clone()
    }
}
impl IdProvider for NanoIdProvider {
    fn provide(&self) -> String {
        nanoid::nanoid!(7)
    }
}
