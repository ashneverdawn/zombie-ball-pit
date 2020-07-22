use amethyst::{
    core::math,
    assets::{AssetStorage, Handle, Loader},
    core::{Named, Parent, Transform, TransformBundle, math::Vector3},
    derive::SystemDesc,
    ecs::{
        Component, Entity, Join, NullStorage, Read, Write, WriteExpect, ReadStorage, System, SystemData, WorldExt,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
    core::timing::Time,
};
use amethyst_physics::{
    prelude::*,
};
use std::ops::Deref;

use crate::systems::player::Player;
use crate::systems::zombie_ai::Zombie;

#[derive(Default)]
pub struct Ball;
impl Component for Ball {
    type Storage = NullStorage<Self>;
}

#[derive(SystemDesc)]
pub struct BallAiSystem;
impl<'s> System<'s> for BallAiSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteExpect<'s, PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Zombie>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (time, physics, rigidbodies, transforms, balls, zombies, players): Self::SystemData) {
        let zero_vec = Vector3::new(0.0, 0.0, 0.0); 
        
        for (my_rigidbody, my_transform, _, _, _) in (&rigidbodies, &transforms, &balls, !&zombies, !&players).join() {
            let mut closest_vec = zero_vec;
            for (other_transform, _) in (&transforms, &balls).join() {
                let other_vec = other_transform.translation() - my_transform.translation();
                if other_vec != zero_vec && (closest_vec == zero_vec || other_vec.magnitude() < closest_vec.magnitude()) {
                    closest_vec = other_vec;
                }
            }

            if closest_vec != zero_vec {
                closest_vec = closest_vec.normalize() * -3000.0 * time.delta_seconds();
                physics.rigid_body_server().apply_force(my_rigidbody.get(), &closest_vec);
            }
            
        }
    }
}