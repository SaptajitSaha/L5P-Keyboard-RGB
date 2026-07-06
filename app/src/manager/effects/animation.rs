use std::{thread, time::Duration};

pub fn lerp(a: u8, b: u8, t: f32) -> u8 {
    ((a as f32) + ((b as f32 - a as f32) * t))
        .round()
        .clamp(0.0, 255.0) as u8
}

pub fn transition(
    from: &[u8; 12],
    to: &[u8; 12],
    frames: u8,
    delay_ms: u64,
    mut draw: impl FnMut([u8; 12]),
) {
    for step in 1..=frames {
        let t = step as f32 / frames as f32;

        let mut rgb = [0u8; 12];

        for i in 0..12 {
            rgb[i] = lerp(from[i], to[i], t);
        }

        draw(rgb);

        thread::sleep(Duration::from_millis(delay_ms));
    }
}

pub fn animation_delay(speed: u8) -> Duration {
    Duration::from_millis(match speed {
        1 => 80,
        2 => 60,
        3 => 40,
        4 => 20,
        5 => 5,
        _ => 40,
    })
}

pub fn apply_brightness(mut rgb: [u8; 12], brightness_level: u8) -> [u8; 12] {
    let factor = match brightness_level {
        1 => 0.25,
        2 => 0.45,
        3 => 0.70,
        _ => 1.00,
    };

    for value in rgb.iter_mut() {
        *value = ((*value as f32) * factor)
            .round()
            .clamp(0.0, 255.0) as u8;
    }

    rgb
}