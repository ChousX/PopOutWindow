use PopOutWindow::{plugins::*, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(GOLD))
        .add_plugins(DefaultPlugins)
        .run();
}
