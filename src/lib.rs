use miniquad::*;

mod draw;
mod shader;

mod render;
use render::*;

mod object;
//use object::*;

pub mod app;
pub use app::*;

pub mod module;
pub use module::*;

pub mod object2d;
pub use object2d::*;

pub mod object3d;
//pub use object3d::*;

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
[?] Добавить мобули для 2д объектов
[] Рисование 2д объектов
[] Создать info
[] Добавление 3д объектов
[] Добавить мобули для 3д объектов
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
[] Кроссплатформенность

Структура:
lib - связь между устройством и движком {
    render: miniquad - рендер {
        shader
    }
    app - состояние движка {
        audio
        object
        shape
        model
        resource
    }
}
singletoon {
    cross: rayon
    module
    utils: glam
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

        start(conf, || Box::new(Event::new(self.app, self.modules)));
    }
}

struct Event {
    app: App,
    modules: ModulesEngine,
    render: Render,
}

impl Event {
    pub fn new(mut app: App, modules: ModulesEngine) -> Self {
        app.post_update();

        Self {
            app,
            modules,
            render: Render::new(),
        }
    }
}

impl EventHandler for Event {
    // Работает и в фоне
    fn update(&mut self) {
        self.app.pre_update(date::now()); // Обновление состояния
        self.modules.update(&mut self.app); // Обновление модулей движка
        self.app.post_update(); // Обновление модулей объектов
    }

    // Только при открытом окне
    fn draw(&mut self) {
        self.render.pre_update(); // Ставит uniforms
        self.render.post_update(); // Коммит фрейма
    }
}
