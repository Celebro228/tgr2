use tgr2::*;


fn main() {
    let mut engine = Engine::new();

    engine.modules.add(Info);
    engine.modules.add(Objs2d);

    engine.run("Tarantula");
}

struct Objs2d;
impl ModuleEngine for Objs2d {
    fn ready(&mut self, app: &mut App) {
        let mut factory = Factory2d::default();

        for _ in -10..10 {
            let mut shape = rect(50., 50.);
            shape.modules.add(Shp);
            factory.add(shape);
        }

        app.objects2d.add("name", factory);
    }
    fn procces(&mut self, _app: &mut App) {
        
    }
}

struct Shp;
impl ModuleShape for Shp {
    fn ready(&mut self, _app: &App, _obj: &Shape) {
        println!("Hello!");
    }
    fn procces(&mut self, app: &App, obj: &Shape) {
        *obj.rotation.lock() += app.info.delta;
        obj.position.lock().x = (app.info.delta * 10.).sin() * 100.;
    }
}


struct Info;
impl ModuleEngine for Info {
    fn ready(&mut self, app: &mut App) {
        println!("os: {:?}", app.info.os);
        println!("time: {:?}", app.info.time);
    }
    fn procces(&mut self, app: &mut App) {
        println!("delta: {}, fps: {}", app.info.delta, app.info.fps);
    }
}
