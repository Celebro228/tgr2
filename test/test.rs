use miniquad::KeyCode;
use tgr2::*;


fn main() {
    let mut engine = Engine::new();
    //engine.modules.add(Info);
    //engine.modules.add(Objs3d);
    engine.modules.add(_Objs2d);
    engine.run("Tarantula");
}


const SPEED: f32 = 10.;
const MOUSE_SPEED: f32 = 0.01;


struct Objs3d;
impl ModuleEngine for Objs3d {
    fn ready(&mut self, app: &mut App) {
        let mut factory = Factory::new();

        for _ in 0..1 {
            let mut shape = cube(1., 1., 1.);
            shape.modules.add(Cbe);
            factory.add(shape);
        }

        app.objects3d.add("name", factory);

        app.events.mouse_show(false);
    }
    fn procces(&mut self, _app: &mut App) {
        
    }
}
struct Cbe;
impl ModuleObject for Cbe {
    fn procces(&mut self, app: &App, obj: &Object) {
        //*obj.rotation.lock() += app.info.delta;

        obj.rotation().x += app.events.mouse_delta.y * MOUSE_SPEED;
        obj.rotation().z += app.events.mouse_delta.x * MOUSE_SPEED;

        if app.events.is_key_pressed(KeyCode::W) {
            obj.position().z -= SPEED * app.info.delta;
        }
        if app.events.is_key_pressed(KeyCode::S) {
            obj.position().z += SPEED * app.info.delta;
        }
        if app.events.is_key_pressed(KeyCode::A) {
            obj.position().x -= SPEED * app.info.delta;
        }
        if app.events.is_key_pressed(KeyCode::D) {
            obj.position().x += SPEED * app.info.delta;
        }
    }
}


struct _Objs2d;
impl ModuleEngine for _Objs2d {
    fn ready(&mut self, app: &mut App) {
        let mut factory = Factory::new();

        for _ in 0..1 {
            let mut shape = rect(50., 50.);
            shape.modules.add(_Shp);
            factory.add(shape);
        }

        app.objects2d.add("name", factory);
    }
    fn procces(&mut self, _app: &mut App) {
        
    }
}
struct _Shp;
impl ModuleObject for _Shp {
    fn ready(&mut self, _app: &App, obj: &Object) {
        obj.position().x = 20.;
    }
}