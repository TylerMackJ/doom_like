use crossterm::style::Color;

pub fn get_color(r: u8, g: u8, b: u8, brightness: f64, ray: (f64, f64), height: f64) -> Color {
    let texture_points = [
        (0.1, -0.2), (0.3, -0.4), (0.6, 0.3), (0.5, 0.2), (0.2, 0.6),
        (0.2, -0.4), (0.2, -0.2), (0.3, 0.3), (0.7, 0.6), (0.7, 0.2),
        (0.8, -0.5), (0.4, -0.7), (0.7, 0.3), (0.4, 0.0), (0.8, 0.0),
        (0.5, -0.0), (0.7, -0.4), (0.5, 0.5), (0.1, 0.7), (0.8, 0.9),
        (0.9, -0.6), (0.1, -0.2), (0.2, 0.4), (0.3, 0.4), (0.6, 0.9),
    ];

    for (i, texture_point) in texture_points.iter().enumerate() {
        if (((ray.0 % 1.0) < (texture_point.0 + 0.05) && (ray.0 % 1.0) > (texture_point.0 - 0.05)) ||
           ((ray.1 % 1.0) < (texture_point.0 + 0.05) && (ray.1 % 1.0) > (texture_point.0 - 0.05))) &&
           height < (texture_point.1 + 0.05) && height > (texture_point.1 - 0.05) {
            return Color::Rgb {
                r: (r as f64 * brightness - i as f64) as u8,
                g: (g as f64 * brightness - i as f64) as u8,
                b: (b as f64 * brightness - i as f64) as u8
            };
        }
    }
    Color::Rgb {
        r: (r as f64 * brightness) as u8,
        g: (g as f64 * brightness) as u8,
        b: (b as f64 * brightness) as u8
    }
}