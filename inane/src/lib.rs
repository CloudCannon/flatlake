pub struct InaneState {}

impl InaneState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {
        println!(
            "Inane v{}\n\n🤷\n\nThere is nothing to do.",
            env!("CARGO_PKG_VERSION")
        )
    }
}
