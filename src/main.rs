use bevy::prelude::*;
use card_game::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
