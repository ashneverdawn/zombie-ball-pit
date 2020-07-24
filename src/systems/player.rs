use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        Named, Parent, Transform, TransformBundle,
        math::{
            Vector3,
        },
    },
    derive::SystemDesc,
    ecs::{
        Component, Entity, Join, NullStorage, Read, ReadStorage, WriteExpect, System, SystemData, WorldExt,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
    core::timing::Time,
};
use amethyst_physics::{
    prelude::*,
};
use std::ops::Deref;


#[derive(Default)]
pub struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        WriteExpect<'s, PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (time, input, mut physics, rigidbodies, players): Self::SystemData) {
        
        let x_move = input.axis_value("move_x").unwrap();
        let z_move = input.axis_value("move_z").unwrap();

        for (_, rigidbody) in (&players, &rigidbodies).join() {

            let mut dir = Vector3::<f32>::new(x_move, 0.0, z_move);
            if dir != Vector3::<f32>::new(0.0, 0.0, 0.0) {
                dir = dir.normalize() * -3000.0 * time.delta_seconds();
                physics.rigid_body_server().apply_force(rigidbody.get(), &dir);
            }
            
        }
        
    }
}