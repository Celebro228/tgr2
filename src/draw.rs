use glam::Vec2;


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Vertex {
    pub(crate) pos: Vec2,
    pub(crate) color: Color,
}

impl Vertex {
    pub(crate) fn new(p: Vec2, c: Color) -> Self {
        Self {
            pos: p,
            color: c,
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: 1.,
        }
    }
    #[inline(always)]
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a,
        }
    }

    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let h = h / 60.;
        let x = c * (1. - ((h % 2.) - 1.).abs());
        let m = v - s;

        let (r, g, b) = match h as u32 {
            0 => (c, x, 0.),
            1 => (x, c, 0.),
            2 => (0., c, x),
            3 => (0., x, c),
            4 => (x, 0., c),
            5 => (c, 0., x),
            _ => (0., 0., 0.), // fallback, например если h = NaN
        };

        Self {
            r: r + m,
            g: g + m,
            b: b + m,
            a: 1.,
        }
    }
}