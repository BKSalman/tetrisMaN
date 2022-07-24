use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

enum BlockType {
    IBlock,
    JBlock,
    LBlock,
    OBlock,
    SBlock,
    TBlock,
    ZBlock,
}

#[derive(Component)]
struct FallingBlock {
    block_type: BlockType,
}

#[derive(Component)]
struct FallingTimer {
    timer: Timer
}

struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::Playing, setup_falling_timer);
    }
}

fn setup_falling_timer(mut commands: Commands) {
    commands
        .insert_resource(
            FallingTimer{
                timer: Timer::new(Duration::from_secs(1), true)
            }
        );
}