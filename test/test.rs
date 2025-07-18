#[cfg(test)]
mod tests {
    use tgr2::*;


    #[test]
    fn module() {
        Engine::new()
            .module(ModuleTest { number: 0 })
            .run("Module");
    }

    struct ModuleTest {
        number: usize
    }

    impl Module for ModuleTest {
        fn procces(&mut self, _app: &App) {
            self.number += 1;
            println!("Hello! {}", self.number)
        }
    }


    #[test]
    fn window() {
        Engine::new().run("Window");
    }
}