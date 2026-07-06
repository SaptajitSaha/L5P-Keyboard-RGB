use std::{sync::atomic::Ordering, thread};

use crate::manager::{
    effects::{
        animation::{animation_delay, apply_brightness, transition, transition_timing},
        frame::Frame,
    },
    profile::Profile,
    Inner,
};

fn set_profile_zone(frame: &mut Frame, profile: &Profile, zone: usize, brightness: f32) {
    if zone >= 4 {
        return;
    }

    if !profile.rgb_zones[zone].enabled {
        return;
    }

    frame.set_zone(zone, profile.rgb_zones[zone].rgb, brightness);
}

pub fn play(inner: &mut Inner, profile: &Profile) {
    let (transition_frames, transition_delay) = transition_timing(profile.speed);
    let mut head: i32 = 0;
    let mut direction: i32 = 1;

    let mut previous = [0u8; 12];

    loop {
        let mut frame = Frame::new();

        // Bright head, using that zone's own color.
        set_profile_zone(&mut frame, profile, head as usize, 1.0);

        // Fading trail behind, also using each zone's own color.
        if direction == 1 {
            if head > 0 {
                set_profile_zone(&mut frame, profile, (head - 1) as usize, 0.55);
            }

            if head > 1 {
                set_profile_zone(&mut frame, profile, (head - 2) as usize, 0.25);
            }
        } else {
            if head < 3 {
                set_profile_zone(&mut frame, profile, (head + 1) as usize, 0.55);
            }

            if head < 2 {
                set_profile_zone(&mut frame, profile, (head + 2) as usize, 0.25);
            }
        }

        transition(&previous, &frame.rgb, transition_frames, transition_delay, |rgb| {
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