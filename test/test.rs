fn main() {
    tests::_shapes();
}


mod tests {
    use tgr2::*;


    pub fn _shapes() {
        Engine::new()
            .module(_ShapesTest)
            .run("Shapes");
    }

    struct _ShapesTest;

    impl Module for _ShapesTest {
        fn ready(&mut self, app: &App) {
            let circle = rect(50., 50.);

            app.objects2d.add("test_circle", circle);
            app.objects2d.add("test_rect", rect(25., 25.));
        }
    }


    pub fn _module() {
        Engine::new()
            .module(_ModuleTest { number: 0 })
            .run("Module");
    }

    struct _ModuleTest {
        number: usize
    }

    impl Module for _ModuleTest {
        fn procces(&mut self, _app: &App) {
            self.number += 1;
            println!("Hello! {}", self.number)
        }
    }


    pub fn _window() {
        Engine::new().run("Window");
    }
}