use amethyst::{
    prelude::*, 
    assets::AssetLoaderSystemData,
    core::Transform,
    renderer::{
        rendy::mesh::{Position, Normal, Tangent, TexCoord},
        shape::Shape, Mesh, Material, MaterialDefaults,
    },
};
use amethyst_physics::{
    prelude::*,
};
use rand;
use crate::systems::ball_ai::Ball;

pub fn create(world : &mut World, y: f32, is_static: bool) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(1.0 / rand::random::<i16>() as f32 - 0.5, 15.0 + y, 1.0 / rand::random::<i16>() as f32 - 0.5);
    
    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        },
    );

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::IcoSphere(Some(2))
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let rigidbody = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Dynamic;
        rb_desc.mass = 1.0;
        rb_desc.bounciness = 0.0;
        rb_desc.friction = 0.05;
        if is_static {
            rb_desc.mode = BodyMode::Static;
        } 
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };
    let collider = {
        let s_desc = ShapeDesc::Sphere {
            radius: 1.0,
        };
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&s_desc)
    };

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .with(rigidbody)
        .with(collider)
        .with(Ball::default())
        .build();
}