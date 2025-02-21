use bevy::prelude::*;

pub fn has_window(window: Option<Single<&Window>>) -> bool {
    window.is_some()
}
