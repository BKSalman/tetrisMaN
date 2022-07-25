use bevy::{prelude::*, window::PresentMode, render::camera::ScalingMode};
use bevy_asset_loader::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
use bevy_embedded_assets::*;

mod plugins;

use plugins::{map::MapPlugin, blocks::BlocksPlugin};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
enum GameState {
    Loading,
    MainMenu,
    Playing,
}

#[derive(AssetCollection)]
pub struct MyAssets {
    #[asset(path = "block.png")]
    pub block: Handle<Image>,
}

const HEIGHT: f32 = 640.;
const RESOLUTION: f32 = 16. / 9.;

fn main() {
    let mut app = App::new();
    app.add_loopless_state(GameState::Loading);
    
    AssetLoader::new(GameState::Loading)
        // https://github.com/NiklasEi/bevy_asset_loader/issues/54
        .continue_to_state(GameState::Playing)
        .with_collection::<MyAssets>()
        .build(&mut app);

    app
        .insert_resource(WindowDescriptor {
                height: HEIGHT,
                width: HEIGHT * RESOLUTION,
                position: Some(Vec2::new(200., 20.)),
                title: "TetrisMaN".into(),
                present_mode: PresentMode::Fifo,
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#bevy-canvas".to_string()),
                resizable: false,
                ..Default::default()
            })
        .add_plugin(ProgressPlugin::new(GameState::Loading))
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        })
        .add_plugin(BlocksPlugin)
        .add_plugin(MapPlugin)
        .insert_resource(ClearColor(Color::rgba(0.38, 0.39, 0.44, 1.)))
        .add_enter_system(GameState::Playing, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    
    camera.orthographic_projection.top = 1000.;
    camera.orthographic_projection.bottom = -1000.;
    camera.orthographic_projection.right = 1000. * RESOLUTION;
    camera.orthographic_projection.left = -1000. * RESOLUTION;     

    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    
    commands.spawn_bundle(camera);
}
