use std::{sync::atomic::Ordering, thread};

use crate::manager::{
    effects::{
        animation::{animation_delay, apply_brightness, transition, transition_timing},
        frame::Frame,
    },
    profile::Profile,
    Inner,
};

const SNAKE_FRAMES: [[f32; 4]; 8] = [
    [1.00, 0.00, 0.00, 0.00],
    [1.00, 0.75, 0.00, 0.00],
    [1.00, 0.75, 0.45, 0.00],
    [1.00, 0.75, 0.45, 0.20],
    [0.00, 1.00, 0.75, 0.45],
    [0.00, 0.00, 1.00, 0.75],
    [0.00, 0.00, 0.00, 1.00],
    [0.00, 0.00, 0.00, 0.00],
];

pub fn play(inner: &mut Inner, profile: &Profile) {
    let (transition_frames, transition_delay) = transition_timing(profile.speed);

    let colors = profile.rgb_zones;
    let mut previous = [0u8; 12];

    loop {
        for brightnesses in SNAKE_FRAMES {
            let mut frame = Frame::new();

            for zone in 0..4 {
                if colors[zone].enabled && brightnesses[zone] > 0.0 {
                    frame.set_zone(zone, colors[zone].rgb, brightnesses[zone]);
                }
            }

            transition(&previous, &frame.rgb, transition_frames, transition_delay, |rgb| {
    let rgb = apply_brightness(rgb, profile.brightness as u8 + 1);
    inner.keyboard.set_colors_to(&rgb).unwrap();
});

            previous = frame.rgb;

            if inner.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                return;
            }

            thread::sleep(animation_delay(profile.speed));
        }
    }
}