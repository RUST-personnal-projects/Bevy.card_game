use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Playing)
            .load_collection::<CardImageAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct CardImageAssets {
    // Special cards:
    #[asset(path = "images/cards/card_back.png")]
    pub card_back: Handle<Image>,
    #[asset(path = "images/cards/4_plus.png")]
    pub plus_4: Handle<Image>,
    #[asset(path = "images/cards/color_change.png")]
    pub change_color: Handle<Image>,
    // Blue cards:
    #[asset(path = "images/cards/blue/0_blue.png")]
    pub blue_0: Handle<Image>,
    #[asset(path = "images/cards/blue/1_blue.png")]
    pub blue_1: Handle<Image>,
    #[asset(path = "images/cards/blue/2_blue.png")]
    pub blue_2: Handle<Image>,
    #[asset(path = "images/cards/blue/3_blue.png")]
    pub blue_3: Handle<Image>,
    #[asset(path = "images/cards/blue/4_blue.png")]
    pub blue_4: Handle<Image>,
    #[asset(path = "images/cards/blue/5_blue.png")]
    pub blue_5: Handle<Image>,
    #[asset(path = "images/cards/blue/6_blue.png")]
    pub blue_6: Handle<Image>,
    #[asset(path = "images/cards/blue/7_blue.png")]
    pub blue_7: Handle<Image>,
    #[asset(path = "images/cards/blue/8_blue.png")]
    pub blue_8: Handle<Image>,
    #[asset(path = "images/cards/blue/9_blue.png")]
    pub blue_9: Handle<Image>,
    #[asset(path = "images/cards/blue/block_blue.png")]
    pub blue_block: Handle<Image>,
    #[asset(path = "images/cards/blue/inverse_blue.png")]
    pub blue_inverse: Handle<Image>,
    #[asset(path = "images/cards/blue/2plus_blue.png")]
    pub blue_plus_two: Handle<Image>,
    // Green cards:
    #[asset(path = "images/cards/green/0_green.png")]
    pub green_0: Handle<Image>,
    #[asset(path = "images/cards/green/1_green.png")]
    pub green_1: Handle<Image>,
    #[asset(path = "images/cards/green/2_green.png")]
    pub green_2: Handle<Image>,
    #[asset(path = "images/cards/green/3_green.png")]
    pub green_3: Handle<Image>,
    #[asset(path = "images/cards/green/4_green.png")]
    pub green_4: Handle<Image>,
    #[asset(path = "images/cards/green/5_green.png")]
    pub green_5: Handle<Image>,
    #[asset(path = "images/cards/green/6_green.png")]
    pub green_6: Handle<Image>,
    #[asset(path = "images/cards/green/7_green.png")]
    pub green_7: Handle<Image>,
    #[asset(path = "images/cards/green/8_green.png")]
    pub green_8: Handle<Image>,
    #[asset(path = "images/cards/green/9_green.png")]
    pub green_9: Handle<Image>,
    #[asset(path = "images/cards/green/block_green.png")]
    pub green_block: Handle<Image>,
    #[asset(path = "images/cards/green/inverse_green.png")]
    pub green_inverse: Handle<Image>,
    #[asset(path = "images/cards/green/2plus_green.png")]
    pub green_plus_two: Handle<Image>,
    // Red cards:
    #[asset(path = "images/cards/red/0_red.png")]
    pub red_0: Handle<Image>,
    #[asset(path = "images/cards/red/1_red.png")]
    pub red_1: Handle<Image>,
    #[asset(path = "images/cards/red/2_red.png")]
    pub red_2: Handle<Image>,
    #[asset(path = "images/cards/red/3_red.png")]
    pub red_3: Handle<Image>,
    #[asset(path = "images/cards/red/4_red.png")]
    pub red_4: Handle<Image>,
    #[asset(path = "images/cards/red/5_red.png")]
    pub red_5: Handle<Image>,
    #[asset(path = "images/cards/red/6_red.png")]
    pub red_6: Handle<Image>,
    #[asset(path = "images/cards/red/7_red.png")]
    pub red_7: Handle<Image>,
    #[asset(path = "images/cards/red/8_red.png")]
    pub red_8: Handle<Image>,
    #[asset(path = "images/cards/red/9_red.png")]
    pub red_9: Handle<Image>,
    #[asset(path = "images/cards/red/block_red.png")]
    pub red_block: Handle<Image>,
    #[asset(path = "images/cards/red/inverse_red.png")]
    pub red_inverse: Handle<Image>,
    #[asset(path = "images/cards/red/2plus_red.png")]
    pub red_plus_two: Handle<Image>,
    // Yellow cards:
    #[asset(path = "images/cards/yellow/0_yellow.png")]
    pub yellow_0: Handle<Image>,
    #[asset(path = "images/cards/yellow/1_yellow.png")]
    pub yellow_1: Handle<Image>,
    #[asset(path = "images/cards/yellow/2_yellow.png")]
    pub yellow_2: Handle<Image>,
    #[asset(path = "images/cards/yellow/3_yellow.png")]
    pub yellow_3: Handle<Image>,
    #[asset(path = "images/cards/yellow/4_yellow.png")]
    pub yellow_4: Handle<Image>,
    #[asset(path = "images/cards/yellow/5_yellow.png")]
    pub yellow_5: Handle<Image>,
    #[asset(path = "images/cards/yellow/6_yellow.png")]
    pub yellow_6: Handle<Image>,
    #[asset(path = "images/cards/yellow/7_yellow.png")]
    pub yellow_7: Handle<Image>,
    #[asset(path = "images/cards/yellow/8_yellow.png")]
    pub yellow_8: Handle<Image>,
    #[asset(path = "images/cards/yellow/9_yellow.png")]
    pub yellow_9: Handle<Image>,
    #[asset(path = "images/cards/yellow/block_yellow.png")]
    pub yellow_block: Handle<Image>,
    #[asset(path = "images/cards/yellow/inverse_yellow.png")]
    pub yellow_inverse: Handle<Image>,
    #[asset(path = "images/cards/yellow/2plus_yellow.png")]
    pub yellow_plus_two: Handle<Image>,
}
