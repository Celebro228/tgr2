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
            app.objects.add("test_circle", Circle(25.))
                .obj()
                .position_set(vec3(25., 25., 0.));
            app.objects.add("test_rect", Rect(25., 25.))
                .obj()
                
                .position_set(vec3(-25., 25., 0.));
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