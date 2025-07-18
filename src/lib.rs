use miniquad::*;

pub mod app;
pub use app::*;

pub mod module;
pub use module::*;


/*

Todo:
[#] Создание окна
[?] Основная логика
[?] Модули
[] Логика модулей
[] 2д Объекты
[] Рисование квадратика
[] Добавление изображения
[] 3д объекты
[] Рисование куба
[] Добавление моделей
[] Аудио
[] Текст
[] Сохранение данных
[] Создание TGR-CLI

Структура:
lib - связь между устройством и движком
app - состояние движка

*/


pub struct Engine {
    app: App,
    modules: Modules,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            app: App::new(),
            modules: Modules::new(),
        }
    }

    pub fn module(mut self, module: impl Module) -> Self {
        self.modules.add_module(&self.app, module);
        self
    }

    pub fn run(self, title: &str) {
        let conf = conf::Conf {
            window_title: title.to_string(),
            window_resizable: true,
            fullscreen: false,
            high_dpi: true,

            // Msaa
            #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
            sample_count: 4,

            #[cfg(any(target_os = "android", target_os = "ios"))]
            sample_count: 2,

            #[cfg(target_arch = "wasm32")]
            sample_count: 1,
            
            ..Default::default()
        };

        start(conf, || Box::new(self));
    }
}

impl EventHandler for Engine {
    fn update(&mut self) {
        self.app = self.modules.update(&mut self.app);  
    }

    fn draw(&mut self) {
        
    }
}
