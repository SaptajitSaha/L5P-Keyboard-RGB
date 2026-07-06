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
    let speed = speed.clamp(1, 10);

    Duration::from_millis(match speed {
        1 => 300,
        2 => 240,
        3 => 180,
        4 => 130,
        5 => 90,
        6 => 55,
        7 => 30,
        8 => 15,
        9 => 5,
        10 => 0,
        _ => 90,
    })
}

pub fn transition_timing(speed: u8) -> (u8, u64) {
    let speed = speed.clamp(1, 10);

    match speed {
        1 => (20, 26),
        2 => (18, 22),
        3 => (16, 18),
        4 => (14, 15),
        5 => (12, 12),
        6 => (10, 10),
        7 => (8, 8),
        8 => (6, 6),
        9 => (5, 5),
        10 => (4, 4),
        _ => (12, 12),
    }
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