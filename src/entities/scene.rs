use amethyst::{
    prelude::*, 
    assets::{Handle, Prefab, PrefabData, PrefabLoader, RonFormat, ProgressCounter},
    controls::ControlTagPrefab,
    derive::PrefabData,
    core::transform::Transform,
    ecs::{Entity, Write},
    renderer::{camera::CameraPrefab, light::LightPrefab},
    utils::auto_fov::AutoFov,
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct Scene {
    handle: Option<Handle<Prefab<ScenePrefabData>>>,
}

#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
pub struct ScenePrefabData {
    transform: Option<Transform>,
    camera: Option<CameraPrefab>,
    auto_fov: Option<AutoFov>,
    light: Option<LightPrefab>,
    fly_tag: Option<ControlTagPrefab>,
}

pub fn init(world : &mut World, progress : &mut ProgressCounter) {
    world.exec(
        |(loader, mut scene): (PrefabLoader<'_, ScenePrefabData>, Write<'_, Scene>)| {
            scene.handle = Some(loader.load(
                "prefabs/scene.ron",
                RonFormat,
                progress
            ));
        },
    );
}

pub fn create(world : &mut World) {
    let scene_handle = world.read_resource::<Scene>()
        .handle.as_ref().unwrap().clone();

    world.create_entity()
        .with(scene_handle)
        .build();
}