use uuid::Uuid;

pub struct Transferer {
    ids: Vec<Uuid>,
}

impl Transferer {
    pub fn new() -> Self {
        Transferer { ids: Vec::new() }
    }

    pub async fn contains(&self, id: Uuid) -> bool {
        let vec = self.ids.iter().filter(|uuid| **uuid == id);

        match vec.count() {
            0 => false,
            1 => true,
            _ => todo!(),
        }
    }
}

impl Default for Transferer {
    fn default() -> Self {
        Self::new()
    }
}
