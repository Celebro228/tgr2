fn main() {
    tests::_info();
}


mod tests {
    use tgr2::*;


    pub fn _info() {
        Engine::new()
            .module(_Info)
            .run("info");
    }

    struct _Info;

    impl Module for _Info {
        fn ready(&mut self, app: &App) {
            println!("os: {:?}", app.info.os);
            println!("time: {:?}", app.info.time);
        }

        fn procces(&mut self, app: &App) {
            println!("delta: {}, fps: {}", app.info.delta, app.info.fps);
        }
    }


    pub fn _benchmark() {
        Engine::new()
            .module(_Benchmark)
            .run("benchmark");
    }

    struct _Benchmark;

    impl Module for _Benchmark {
        fn ready(&mut self, app: &App) {
            for i in 0..1000 {
                app.objects2d.add(&i.to_string(), rect(25., 25.));
            }
        }
    }


    pub fn _shapes() {
        Engine::new()
            .module(_ShapesTest)
            .run("Shapes");
    }

    struct _ShapesTest;

    impl Module for _ShapesTest {
        fn ready(&mut self, app: &App) {
            let circle = rect(50., 50.);
            circle.position_set(vec2(25., 25.));
            circle.rotation_set(1.);

            app.objects2d.add("test_circle", circle);
            app.objects2d.add("test_rect", rect(25., 25.));
        }

        fn procces(&mut self, app: &App) {
            let rect = app.objects2d.get("test_rect").unwrap();
            let rot = rect.rotation_get();
            rect.rotation_set(rot + 0.001);
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