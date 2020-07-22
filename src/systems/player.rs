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


#[derive(Default)]
pub struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, input): Self::SystemData) {
        /*
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        for (_, transform) in (&players, &mut transforms).join() {
            transform.prepend_translation_x(x_move as f32 * 5.0);
            transform.prepend_translation_y(y_move as f32 * 5.0);
            // println!("Player = {:?}", transform);
        }
        */
    }
}