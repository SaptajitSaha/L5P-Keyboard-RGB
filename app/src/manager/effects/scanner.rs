use std::{sync::atomic::Ordering, thread, time::Duration};

use crate::manager::{profile::Profile, Inner};

pub fn play(manager: &mut Inner, p: &Profile) {
    let profile_array = p.rgb_array();

    let mut current_zone: i32 = 0;
    let mut direction: i32 = 1;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut arr = [0u8; 12];

        let start = (current_zone as usize) * 3;

        arr[start] = profile_array[start];
        arr[start + 1] = profile_array[start + 1];
        arr[start + 2] = profile_array[start + 2];

        manager.keyboard.set_colors_to(&arr).unwrap();

        thread::sleep(Duration::from_millis(120));

        current_zone += direction;

        if current_zone == 3 {
            direction = -1;
        }

        if current_zone == 0 {
            direction = 1;
        }
    }
}