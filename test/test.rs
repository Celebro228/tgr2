#[cfg(test)]
mod tests {
    use tgr2::*;


    #[test]
    fn shapes() {
        Engine::new()
            .module(ShapesTest)
            .run("Module");
    }

    struct ShapesTest;

    impl Module for ShapesTest {
        fn ready(&mut self, app: &App) {
            app.objects2d.add("test_circle", Circle(25.))
                .obj()
                .position_set(vec2(25., 25.));
            app.objects2d.add("test_rect", Rect(25., 25.))
                .obj()
                .position_set(vec2(-25., 25.));
        }
    }


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