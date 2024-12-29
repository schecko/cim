
mod board_vis_tuning;

use base::array2::*;
use base::extents::*;
use base::ronx::*;
use bevyx::ron::RonAssetPlugin;
use board_vis_tuning::*;

use base::tuning::Tuning;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::*;
use bitflags::bitflags;

pub fn hello_vis()
{
    println!("Hello, vis!");
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial
{
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl Material2d for CustomMaterial
{
    fn fragment_shader() -> ShaderRef
    {
        "shaders/custom_material.wgsl".into()
    }
}

bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CellType: u8
    {
        const None = 0 << 0;
        const Land = 1 << 0;
    }
}

#[derive(Debug, Clone, Resource)]
struct BoardVis
{
    cell_type: Array2<CellType>,
}

#[derive(Debug, Clone, Copy, Component)]
struct VisCell
{
    index: usize,
    pos: (usize, usize),
}

fn pre_startup
(
    mut commands: Commands,
)
{
    commands.insert_resource(BoardVisTuning::load());
}

fn startup
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    vis_tuning: Res<BoardVisTuning>,
    asset_server: Res<AssetServer>,
)
{
    let size = Extents::new(10, 10);
    let mut vis = BoardVis
    {
        cell_type: Array2::<CellType>::from_size(size),
    };

    let custom_material = materials.add
    (
        CustomMaterial
        {
            color: Color::WHITE.into(),
            color_texture: asset_server.load("textures/sample.png"),
        }
    );

    let v_pos = vec!
    [
        [0.0, 0.0, 0.0], // TL
        [1.0, 0.0, 0.0], // TR
        [0.0, 1.0, 0.0], // BL
        [1.0, 1.0, 0.0], // BR
    ];
    let v_color: Vec<[f32; 4]> = vec![LinearRgba::WHITE.to_f32_array(); 4];
    let v_uv: Vec<[f32; 2]> = vec!
    [
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];
    let v_normal: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

    let indices = vec!
    [
        0, 2, 1,
        1, 2, 3
    ];

    let mut mesh = Mesh::new
    (
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_normal);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    let mesh_id = meshes.add(mesh);

    for pos in size.positions_row_major()
    {
        let scale = vis_tuning.cell_size;
        let translation = Vec2::new(pos.0 as f32, -(pos.1 as f32)) * scale; 
        commands
            .spawn
            ((
                Mesh2d(mesh_id.clone().into()),
                MeshMaterial2d(custom_material.clone().into()),
                Transform::default()
                    .with_translation(translation.extend(0.0))
                    .with_scale(scale.extend(0.0)),
            ))
            .insert(VisCell{ index: size.get_index_row_major(pos.0, pos.1).unwrap(), pos });
    }
}

pub struct GameVisPlugin;

impl Plugin for GameVisPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(RonAssetPlugin::<BoardVisTuning>::default())
            .add_plugins(Material2dPlugin::<CustomMaterial>::default())
            .add_systems(PreStartup, pre_startup)
            .add_systems(Startup, startup);
    }
}
