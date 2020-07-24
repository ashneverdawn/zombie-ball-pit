use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{Named, Parent, Transform, TransformBundle},
    derive::SystemDesc,
    ecs::{
        Component, Entity, Join, NullStorage, Read, ReadStorage, System, SystemData, WorldExt,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
};


#[derive(SystemDesc)]
pub struct GameSystem;

impl<'s> System<'s> for GameSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, input): Self::SystemData) {
        
    }
}