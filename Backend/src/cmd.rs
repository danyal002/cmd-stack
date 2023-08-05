pub struct Cmd {
    id: u32,
    tag: String,
    note: String,
}

impl Cmd {
    pub fn new(id: u32, tag: String, note: String) -> Self {
        Cmd { id, tag, note }
    }
}
