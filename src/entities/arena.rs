use amethyst::{
    prelude::*, 
    assets::{AssetLoaderSystemData, Handle, ProgressCounter},
    core::transform::Transform,
    core::math::{Vector3, Point3},
    renderer::{
        rendy::mesh::{Position, Normal, Tangent, TexCoord, MeshBuilder},
        Mesh, Material, MaterialDefaults, shape::InternalVertexData, visibility::BoundingSphere,
    },
};
use genmesh::{
    generators::{
        IcoSphere, IndexedPolygon, SharedVertex,
    },
    EmitTriangles, MapVertex, Triangulate, Vertex, Vertices,
};
use amethyst_physics::{
    prelude::*,
};

pub fn init(world : &mut World, progress : &mut ProgressCounter) -> (Handle<Mesh>, Vec<Point3<f32>>, Vec<Point3::<usize>>) {

    let shape = generate_vertices(
        Some(3)
            .map(IcoSphere::subdivide)
            .unwrap_or_else(IcoSphere::new),
        Some((25.0, 25.0, 25.0)),
    );

    let mut data = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for v in shape.iter().rev() {
        data.0.push(Position(v.0));
        data.1.push(Normal([-v.1[0], -v.1[1], -v.1[2]]));
        data.2.push(Tangent([v.3[0], v.3[1], v.3[2], 1.0]));
        data.3.push(TexCoord(v.2));
    }
    let vertex_count = data.0.len();

    let mut collider_mesh = (Vec::new(), Vec::new());
    
    for v in &data.0 {
        collider_mesh.0.push(Point3::<f32>::new(v.0[0], v.0[1], v.0[2]));
    }

    let mut i = 2; 
    while i < vertex_count {
        collider_mesh.1.push(Point3::<usize>::new(i-2, i-1, i));
        i += 3;
    }

    let mesh : MeshBuilder = <(Vec::<Position>, Vec::<Normal>, Vec::<Tangent>, Vec::<TexCoord>)>::from(data).into();

    return (world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            mesh.into(),
            progress,
        )
    }), collider_mesh.0, collider_mesh.1);
}

pub fn create(world : &mut World, mesh_data : (Handle<Mesh>, Vec<Point3<f32>>, Vec<Point3::<usize>>)) {
    let (mesh, collider_vertices, collider_indices) = mesh_data;
    
    let transform = Transform::default();

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

    let collider = {
        let s_desc = ShapeDesc::TriMesh {
            points: collider_vertices, 
            indices: collider_indices, 
        };
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&s_desc)
    };
    
    let rigidbody = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;
        rb_desc.mass = 1.0;
        rb_desc.bounciness = 0.0;
        rb_desc.friction = 0.05;
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    let t = transform.translation();
    let pos = Point3::<f32>::new(t[0], t[1], t[2]);
    let bounding_sphere = BoundingSphere::new(pos, 25.0);

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(bounding_sphere)
        .with(transform)
        .with(rigidbody)
        .with(collider)
        .build();
}


fn generate_vertices<F, P, G>(
    generator: G,
    scale: Option<(f32, f32, f32)>,
) -> Vec<InternalVertexData>
where
    F: EmitTriangles<Vertex = Vertex>,
    F::Vertex: Clone + Copy + PartialEq,
    P: EmitTriangles<Vertex = usize>,
    G: SharedVertex<F::Vertex> + IndexedPolygon<P> + Iterator<Item = F>,
{
    let vertices = generator.shared_vertex_iter().collect::<Vec<_>>();
    generator
        .indexed_polygon_iter()
        .triangulate()
        .map(|f| {
            f.map_vertex(|u| {
                let v = vertices[u];
                let pos = scale
                    .map(|(x, y, z)| Vector3::new(v.pos.x * x, v.pos.y * y, v.pos.z * z))
                    .unwrap_or_else(|| Vector3::from(v.pos));
                let normal = scale
                    .map(|(x, y, z)| {
                        Vector3::new(v.normal.x * x, v.normal.y * y, v.normal.z * z).normalize()
                    })
                    .unwrap_or_else(|| Vector3::from(v.normal));
                let tangent1 = normal.cross(&Vector3::x());
                let tangent2 = normal.cross(&Vector3::y());
                let tangent = if tangent1.norm_squared() > tangent2.norm_squared() {
                    tangent1
                } else {
                    tangent2
                }
                .cross(&normal);

                (
                    pos.into(),
                    normal.into(),
                    [(v.pos.x + 1.) / 2., (v.pos.y + 1.) / 2.],
                    tangent.into(),
                )
            })
        })
        .vertices()
        .collect::<Vec<_>>()
}