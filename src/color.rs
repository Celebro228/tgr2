pub use wgpu::Color;


#[inline(always)]
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f64 / 255.,
        g: g as f64 / 255.,
        b: b as f64 / 255.,
        a: 1.,
    }
}

#[inline(always)]
pub fn rgba(r: u8, g: u8, b: u8, a: f64) -> Color {
    Color {
        r: r as f64 / 255.,
        g: g as f64 / 255.,
        b: b as f64 / 255.,
        a,
    }
}

pub fn hsv(h: f64, s: f64, v: f64) -> Color {
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

    Color {
        r: r + m,
        g: g + m,
        b: b + m,
        a: 1.,
    }
}