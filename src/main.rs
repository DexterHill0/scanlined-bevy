use bevy::prelude::*;

use scanlined_bevy::ScanlinedApp;

fn main() {
    let mut app = App::new();

    app.add_plugins(ScanlinedApp);

    app.run();
}
