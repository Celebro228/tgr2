use miniquad::KeyCode;
use tgr2::*;


fn main() {
    let mut engine = Engine::new();
    //engine.modules.add(Info);
    engine.modules.add(Objs3d);
    //engine.modules.add(Objs2d);
    engine.run("Tarantula");
}


const SPEED: f32 = 10.;


struct Objs3d;
impl ModuleEngine for Objs3d {
    fn ready(&mut self, app: &mut App) {
        let mut factory = Factory3d::default();

        for _ in 0..1 {
            let mut shape = cube(1., 1., 1.);
            shape.modules.add(Cbe);
            factory.add(shape);
        }

        app.objects3d.add("name", factory);
    }
    fn procces(&mut self, _app: &mut App) {
        
    }
}
struct Cbe;
impl ModuleModel for Cbe {
    fn ready(&mut self, _app: &App, obj: &Model) {
        
    }
    fn procces(&mut self, app: &App, obj: &Model) {
        *obj.rotation.lock() += app.info.delta;
        if app.events.is_key_pressed(KeyCode::W) {
            obj.position.lock().z -= SPEED * app.info.delta;
        }
        if app.events.is_key_pressed(KeyCode::S) {
            obj.position.lock().z += SPEED * app.info.delta;
        }
        if app.events.is_key_pressed(KeyCode::A) {
            obj.position.lock().x -= SPEED * app.info.delta;
        }
        if app.events.is_key_pressed(KeyCode::D) {
            obj.position.lock().x += SPEED * app.info.delta;
        }
    }
}


struct Objs2d;
impl ModuleEngine for Objs2d {
    fn ready(&mut self, app: &mut App) {
        let mut factory = Factory2d::default();

        for _ in 0..10000 {
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
    fn ready(&mut self, _app: &App, obj: &Shape) {
        obj.position.lock().x = 20.;
    }
    fn procces(&mut self, app: &App, obj: &Shape) {
        
    }
}