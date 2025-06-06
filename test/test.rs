fn main() {
    window::main();
}


mod window {
    use tgr2::*;

    pub fn main() {
        Engine::new("Ok").run();
    }
}