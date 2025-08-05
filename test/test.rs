fn main() {
    tests::_shapes();
}

mod tests {
    use tgr2::*;

    /*pub fn _model() {
        Engine::new()
            .module(_Model)
            .run("model");
    }
    struct _Model;
    impl Module for _Model {
        fn ready(&mut self, app: &App) {
            let cube = cube(2., 2., 2.);
            app.objects3d.add("test_cube", cube);
        }
        fn procces(&mut self, app: &App) {
            let cube = app.objects3d.get("test_rect").unwrap();
            let mut rot = cube.rotation_get();
            rot.x += app.info.delta;
            cube.rotation_set(rot);
        }
    }*/

    pub fn _info() {
        Engine::new().module(_Info).run("info");
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
        Engine::new().module(_Benchmark).run("benchmark");
    }
    struct _Benchmark;
    impl Module for _Benchmark {
        fn ready(&mut self, app: &App) {
            for i in 0..1000 {
                app.objects2d.add(&i.to_string(), rect(25., 25.));
            }
        }
        fn procces(&mut self, app: &App) {
            println!("{}", 1. / app.info.delta);
        }
    }

    pub fn _shapes() {
        Engine::new().module(_ShapesTest).run("Shapes");
    }
    struct _ShapesTest;
    impl Module for _ShapesTest {
        fn ready(&mut self, app: &App) {
            let circle = rect(50., 50.);
            circle.position.set(vec2(25., 25.));
            circle.rotation.set(1.);

            app.objects2d.add("test_circle", circle);
            app.objects2d.add("test_rect", rect(25., 25.));
        }
        fn procces(&mut self, app: &App) {
            let rect = app.objects2d.get("test_rect").unwrap();
            let rot = *rect.rotation.lock();
            rect.rotation.set(rot + app.info.delta);
        }
    }

    pub fn _module() {
        Engine::new()
            .module(_ModuleTest { number: 0 })
            .run("Module");
    }
    struct _ModuleTest {
        number: usize,
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
