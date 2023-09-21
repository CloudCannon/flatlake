pub struct Watershed {}

impl Watershed {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {
        println!("flatlake running as {}", env!("CARGO_PKG_VERSION"))
    }
}
