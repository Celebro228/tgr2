use miniquad::*;
use std::any::Any;

mod shader;

mod render;
use render::*;

mod object;
//use object::*;

pub mod draw;
pub use draw::*;

pub mod app;
pub use app::*;

pub mod object2d;
pub use object2d::*;

pub mod object3d;
pub use object3d::*;

pub mod info;
pub use info::*;

pub mod cross;
pub use cross::*;

pub mod event;
pub use event::*;



/*

TODO:
[#] Создание окна
[#] Основная логика
[#] Модули
[#] Логика модулей
[#] Добавление 2д объектов
[#] Добавить мобули для 2д объектов
[#] Рисование 2д объектов
[#] Создать info
[#] Добавление 3д объектовя
[#] Добавить мобули для 3д объектов
[#] Рисование 3д объектов
[-] Сделать общий transform
[#] Разделить цвета от вершин
[?] Создать ивенты
[] Создать 2д камеру
[] Создать 3д камеру
[] Создать resource
[] Добавление изображения
[] Модели
[] Свет
[] Аудио
[] Добавление текста
[] Сохранение данных
[] Создание TGR-CLI
[] Мультиплеер
[] Кроссплатформенность

Структура:
lib - связь между устройством и движком (ивенты) {
    render: miniquad - рендер {
        shader: miniquad
    }
    app - состояние движка {
        audio
        object
        shape
        model
        resource
        event: miniquad
    }
}
singletoon {
    cross: rayon - кроссплатформенные функции
    draw
}

*/



pub struct Engine {
    pub app: App,
    pub modules: ModulesEngine,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            app: App::new(date::now()),
            modules: ModulesEngine::default(),
        }
    }

    pub fn run(self, title: &str) {
        let conf = conf::Conf {
            window_title: title.to_string(),
            high_dpi: true,

            // Окно в режиме дебага
            #[cfg(debug_assertions)]
            window_resizable: true,
            #[cfg(debug_assertions)]
            fullscreen: false,

            // Окно в режиме релиза
            #[cfg(not(debug_assertions))]
            window_resizable: false,
            #[cfg(not(debug_assertions))]
            fullscreen: true,

            // Msaa
            #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
            sample_count: 4,
            #[cfg(any(target_os = "android", target_os = "ios"))]
            sample_count: 2,
            #[cfg(target_arch = "wasm32")]
            sample_count: 1,

            ..Default::default()
        };

        start(conf, || Box::new(EventEngine::new(self.app, self.modules)));
    }
}

struct EventEngine {
    app: App,
    modules: ModulesEngine,
    render: Render,
}

impl EventEngine {
    pub fn new(mut app: App, modules: ModulesEngine) -> Self {
        app.post_update();

        Self {
            app,
            modules,
            render: Render::new(),
        }
    }
}

impl EventHandler for EventEngine {
    // Работает и в фоне
    fn update(&mut self) {
        let t = Timer::new("Info update app");
        self.app.pre_update(date::now()); // Обновление состояния
        t.stop();
        
        let t = Timer::new("Engine modules update");
        self.modules.update(&mut self.app); // Обновление модулей движка
        t.stop();
        
        let t = Timer::new("Objects modules update");
        self.app.post_update(); // Обновление модулей объектов
        t.stop();
    }
    // Только при открытом окне
    fn draw(&mut self) {
        let t = Timer::new("Frame");
        self.render.pre_update(); // Готовит фрейм
        t.stop();
        
        let t = Timer::new("Draw 3d");
        self.app.draw_3d(&mut self.render.ctx); // Рисование 3д
        t.stop();
        
        let t = Timer::new("Draw 2d");
        self.app.draw_2d(&mut self.render.ctx); // Рисование 2д
        t.stop();
        
        let t = Timer::new("Commit");
        self.render.post_update(); // Коммит фрейма
        t.stop();

        println!("Fps: {}", self.app.info.fps as usize);
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if !repeat {
            self.app.event_send_set(EventChange::Press(keycode));
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.app.event_send_set(EventChange::Release(keycode));
    }
}


struct Timer {
    time: std::time::Instant,
    name: String,
}
impl Timer {
    fn new(name: &str) -> Self {
        Self {
            time: std::time::Instant::now(),
            name: name.to_string(),
        }
    }
    fn stop(&self) {
        println!("{}: {:?}", self.name, self.time.elapsed());
    }
}


#[derive(Default)]
pub struct ModulesEngine {
    module_list: Vec<Box<dyn ModuleEngine>>,
    module_list_len: usize,
}
impl ModulesEngine {
    pub(crate) fn update(&mut self, app: &mut App) {
        for module in &mut self.module_list[self.module_list_len..] {
            module.ready(app);
        }
        self.module_list_len = self.module_list.len();

        for module in &mut self.module_list {
            module.procces(app);
        }
    }
    pub fn add(&mut self, module: impl ModuleEngine) {
        self.module_list.push(Box::new(module));
    }
}
pub trait ModuleEngine: Any + Sync + Send {
    fn ready(&mut self, _app: &mut App) {}
    fn procces(&mut self, _app: &mut App) {}
}