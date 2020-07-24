use amethyst::{
    utils::application_root_dir,
    GameDataBuilder, Application,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow}, //RenderSkybox},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::TransformBundle,
    assets::PrefabLoaderSystemDesc,
    controls::FlyControlBundle,
    input::{StringBindings, InputBundle},
};
use amethyst_physics::PhysicsBundle;
use amethyst_nphysics::NPhysicsBackend;
use amethyst_imgui::RenderImgui;

mod states;
mod entities;
mod systems;

fn main() -> amethyst::Result<()> {

    // TODO: remove warn suppression.
    amethyst::start_logger(amethyst::LoggerConfig{
        log_gfx_backend_level: Some(log::LevelFilter::Error),
        ..Default::default()
    });

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
        .with_system_desc(PrefabLoaderSystemDesc::<entities::scene::ScenePrefabData>::default(), "scene_loader", &[])
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
            .with_sensitivity(0.1, 0.1)
            .with_speed(5.),
        )?
        .with_bundle(TransformBundle::new().with_dep(&[
            "fly_movement", "free_rotation",
        ]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(systems::ball_ai::BallAiSystem, "ball_ai", &[])
        .with(systems::zombie_ai::ZombieAiSystem, "zombie_ai", &[])
        .with(systems::player::PlayerSystem, "player", &[])
        .with(systems::game::GameSystem, "game", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]), //rgba background
                )
                .with_plugin(RenderImgui::<amethyst::input::StringBindings>::default())
                .with_plugin(RenderPbr3D::default())
                //.with_plugin(RenderSkybox::default()),
        )?;

    // Run the game!
    let mut game = Application::new(assets_dir, states::loading::LoadingState::default(), game_data)?;
    game.run();

    Ok(())
}