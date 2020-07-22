use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{Named, Parent, Transform, TransformBundle},
    derive::SystemDesc,
    ecs::{
        Component, Entity, Join, NullStorage, Read, ReadStorage, System, SystemData, WorldExt,
        WriteStorage,
    },
};



#[derive(Default)]
pub struct Zombie;
impl Component for Zombie {
    type Storage = NullStorage<Self>;
}
#[derive(SystemDesc)]
pub struct ZombieAiSystem;

impl<'s> System<'s> for ZombieAiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut transforms): Self::SystemData) {

    }
}