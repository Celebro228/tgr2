fn main() {
    test::main();
}


mod test {
    use tgr2::*;

    pub fn main() {
        Engine::new("Ok").run();
    }
}