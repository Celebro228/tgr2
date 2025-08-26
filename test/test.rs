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

        app.node3d.add("name", factory);

        app.events.mouse_show(false);
    }
    fn procces(&mut self, _app: &mut App) {
        
    }
}
struct Cbe;
impl ModuleObject for Cbe {
    fn ready(&mut self, _app: &App, obj: &mut Object) {
        obj.position.z = -10.;
    }
    fn procces(&mut self, app: &App, obj: &mut Object) {
        //*obj.rotation.lock() += app.info.delta;

        obj.rotation.x += app.events.mouse_delta.y * MOUSE_SPEED;
        obj.rotation.z += app.events.mouse_delta.x * MOUSE_SPEED;

        if app.events.is_key_pressed(KeyCode::W) {
            app.camera3d.position().z -= SPEED * app.info.delta();
        }
        if app.events.is_key_pressed(KeyCode::S) {
            app.camera3d.position().z += SPEED * app.info.delta();
        }
        if app.events.is_key_pressed(KeyCode::A) {
            app.camera3d.position().x -= SPEED * app.info.delta();
        }
        if app.events.is_key_pressed(KeyCode::D) {
            app.camera3d.position().x += SPEED * app.info.delta();
        }
    }
}


struct _Objs2d;
impl ModuleEngine for _Objs2d {
    fn ready(&mut self, app: &mut App) {
        let mut factory = Factory::new();

        for _ in 0..10000 {
            let mut shape = rect(50., 50.);
            shape.modules.add(_Shp);
            factory.add(shape);
        }

        app.node2d.add("name", factory);
    }
    fn procces(&mut self, _app: &mut App) {
        
    }
}
struct _Shp;
impl ModuleObject for _Shp {
    fn ready(&mut self, _app: &App, obj: &mut Object) {
        obj.position.x = 20.;
    }
    fn procces(&mut self, app: &App, _obj: &mut Object) {
        if app.events.is_key_pressed(KeyCode::W) {
            app.camera2d.position().y += 1.;
        }
        if app.events.is_key_pressed(KeyCode::S) {
            app.camera2d.position().y -= 1.;
        }
        if app.events.is_key_pressed(KeyCode::A) {
            app.camera2d.position().x -= 1.;
        }
        if app.events.is_key_pressed(KeyCode::D) {
            app.camera2d.position().x += 1.;
        }
    }
}