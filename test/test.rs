fn main() {
    tests::_info();
}

mod tests {
    use tgr2::*;

    pub fn _benchmark() {
        let mut engine = Engine::new();
        engine.modules.add(_Benchmark);
        engine.run("benchmark");
    }
    struct _Benchmark;
    impl ModuleEngine for _Benchmark {
        fn ready(&mut self, app: &mut App) {
            for i in 0..1000 {
                app.objects2d.add(&i.to_string(), rect(25., 25.));
            }
        }
        fn procces(&mut self, app: &mut App) {
            println!("{}", app.info.fps);
        }
    }


    pub fn _info() {
        let mut engine = Engine::new();
        engine.modules.add(_Info);
        engine.run("info");
    }
    struct _Info;
    impl ModuleEngine for _Info {
        fn ready(&mut self, app: &mut App) {
            println!("os: {:?}", app.info.os);
            println!("time: {:?}", app.info.time);
        }
        fn procces(&mut self, app: &mut App) {
            println!("delta: {}, fps: {}, fps_avg: {}", app.info.delta, 1. / app.info.delta, app.info.fps);
        }
    }
}
