use std::{sync::atomic::Ordering, thread};

use crate::manager::{
    effects::{
        animation::{animation_delay, apply_brightness, transition},
        frame::Frame,
    },
    profile::Profile,
    Inner,
};

pub fn play(inner: &mut Inner, profile: &Profile) {
    let color = profile.rgb_zones[0].rgb;

    let mut head: i32 = 0;
    let mut direction: i32 = 1;

    let mut previous = [0u8; 12];

    loop {
        let mut frame = Frame::new();

        // Bright head
        frame.set_zone(head as usize, color, 1.0);

        // Fading trail behind
        if direction == 1 {
            if head > 0 {
                frame.set_zone((head - 1) as usize, color, 0.55);
            }
            if head > 1 {
                frame.set_zone((head - 2) as usize, color, 0.25);
            }
        } else {
            if head < 3 {
                frame.set_zone((head + 1) as usize, color, 0.55);
            }
            if head < 2 {
                frame.set_zone((head + 2) as usize, color, 0.25);
            }
        }

        transition(&previous, &frame.rgb, 8, 12, |rgb| {
    let rgb = apply_brightness(rgb, profile.brightness as u8 + 1);
    inner.keyboard.set_colors_to(&rgb).unwrap();
});

        previous = frame.rgb;

        if inner.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
            break;
        }

        thread::sleep(animation_delay(profile.speed));

        head += direction;

        if head >= 3 {
            direction = -1;
        }

        if head <= 0 {
            direction = 1;
        }
    }
}