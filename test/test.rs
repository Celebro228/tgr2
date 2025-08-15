fn main() {
    tests::_info();
}

mod tests {
    use tgr2::*;


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
            app.objects2d.add("name", Factory2d::new());
        }
        fn procces(&mut self, app: &mut App) {
            println!("delta: {}, fps: {}", app.info.delta, app.info.fps);
        }
    }
}
