pub struct Info {
    time: f64,
    delta: f32,
    fps: f32,
    os: OS,
}

impl Info {
    pub(crate) fn new(time: f64) -> Self {
        #[cfg(target_os = "windows")]
        let os = OS::Windows;
        #[cfg(target_os = "linux")]
        let os = OS::Linux;
        #[cfg(target_os = "macos")]
        let os = OS::Macos;
        #[cfg(target_os = "android")]
        let os = OS::Android;
        #[cfg(target_os = "ios")]
        let os = OS::Ios;
        #[cfg(target_arch = "wasm32")]
        let os = OS::Web;

        Self {
            time,
            delta: 0.,
            fps: 60.,
            os,
        }
    }

    pub(crate) fn update(&mut self, time: f64) {
        self.delta = (time - self.time) as f32;
        self.fps = 1. / self.delta;
        self.time = time;
    }

    pub fn time(&self) -> f64 { self.time }
    pub fn delta(&self) -> f32 { self.delta }
    pub fn fps(&self) -> f32 { self.fps }
    pub fn os(&self) -> &OS { &self.os }
}

#[derive(Debug)]
pub enum OS {
    Windows,
    Linux,
    Macos,
    Android,
    Ios,
    Web,
    Unknow,
}
