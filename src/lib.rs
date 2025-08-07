use miniquad::*;

mod shader;

mod render;
use render::*;

pub mod app;
pub use app::*;

pub mod module;
pub use module::*;

pub mod object2d;
pub use object2d::*;

pub mod shape;
pub use shape::*;

pub mod object3d;
pub use object3d::*;

pub mod model;
//pub use model::*;

pub mod draw;
pub use draw::*;

pub mod info;
pub use info::*;

pub mod cross;
pub use cross::*;

/*

TODO:
[#] Создание окна
[#] Основная логика
[#] Модули
[#] Логика модулей
[#] Добавление 2д объектов
[#] Изменение 2д объектов
[#] Добавить update и/или draw в objects
[#] Рисование 2д объектов
[#] Создать info
[] Добавление 3д объектов
[] Изменение 3д объектов
[] Рисование 3д объектов
[] Создать resource
[] Добавление изображения
[] Модели
[] Свет
[] Аудио
[] Добавление текста
[] Сохранение данных
[] Создание TGR-CLI
[] Мультиплеер

Структура:
lib - связь между устройством и движком {
    module - модули
    render - рендер {
        shader
    }
    app - состояние движка {
        audio
        LData
        object
        shape
        model
        resource
    }
}
singletoon {
    cross
}
*/

pub struct Engine {
    app: App,
    modules: Modules,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            app: App::new(date::now()),
            modules: Modules::default(),
        }
    }

    pub fn module(mut self, module: impl Module) -> Self {
        self.modules.add_module(&self.app, module);
        self
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

        start(conf, || Box::new(Event::new(self.app, self.modules)));
    }
}

struct Event {
    app: App,
    modules: Modules,
    render: Render,
}

impl Event {
    pub fn new(mut app: App, modules: Modules) -> Self {
        app.post_update();

        Self {
            app,
            modules,
            render: Render::new(),
        }
    }
}

impl EventHandler for Event {
    fn update(&mut self) {
        self.app.pre_update(date::now());
        self.modules.update(&mut self.app);
        self.app.post_update();
    }

    fn draw(&mut self) {
        self.render.draw(&self.app);
    }
}
