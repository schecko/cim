
mod board_vis_tuning;

use base::array2::*;
use base::extents::*;
use base::ronx::*;
use base::tuning::Tuning;
use bevyx::ron::RonAssetPlugin;
use board_vis_tuning::*;

use bevy::math::VectorSpace;
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
    #[texture(1, sample_type="float")]
    #[sampler(2, sampler_type="filtering")]
    elevation: Handle<Image>,

    // should be 1d, but bevy can't load png as 1d
    #[texture(3, sample_type="float", dimension="2d")]
    #[sampler(4, sampler_type="filtering")]
    color_palette: Handle<Image>,
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

#[derive(Debug, Resource)]
struct TerrainData
{

}

fn startup
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut images: ResMut<Assets<Image>>,
    vis_tuning: Res<BoardVisTuning>,
    asset_server: Res<AssetServer>,
)
{
    let size = Extents::new(20, 20);
    let mut vis = BoardVis
    {
        cell_type: Array2::<CellType>::from_size(size),
    };

    let elevation_handle = {
        let mut elevation: Vec<u8> = vec![];
        elevation.resize(size.width * size.height * size_of::<u32>(), 0);
        let elevation_slice = bytemuck::cast_slice_mut::<u8, u32>(&mut elevation);
        for i in 0..(size.width * size.height)
        {
            elevation_slice[i] = if i & 1 != 0
            {
                0xFF
            }
            else
            {
                0x0
            };
        }
        let mut elevation_image = Image::new
        (
            Extent3d{ width: size.width as u32, height: size.height as u32, depth_or_array_layers: 1 },
            TextureDimension::D2,
            elevation,
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::RENDER_WORLD
        );
        elevation_image.sampler = bevy::image::ImageSampler::Descriptor
        (
            bevy::image::ImageSamplerDescriptor {
                label: Some("elevation".to_owned()),
                mag_filter: bevy::image::ImageFilterMode::Nearest,
                min_filter: bevy::image::ImageFilterMode::Nearest,
                mipmap_filter: bevy::image::ImageFilterMode::Nearest,
                ..Default::default()
            }
        );

        images.add(elevation_image)
    };

    let custom_material = materials.add
    (
        CustomMaterial
        {
            color: Color::WHITE.into(),
            elevation: elevation_handle,
            color_palette: asset_server.load("textures/palette.png"),
        }
    );

    let scale = vis_tuning.cell_size * Vec2::new(size.width as f32, size.height as f32);
    let v_pos = vec!
    [
        [0.0 * scale.x, 0.0 * scale.y, 0.0], // TL
        [1.0 * scale.x, 0.0 * scale.y, 0.0], // TR
        [0.0 * scale.x, 1.0 * scale.y, 0.0], // BL
        [1.0 * scale.x, 1.0 * scale.y, 0.0], // BR
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

    commands
        .spawn
        ((
            Mesh2d(mesh_id.into()),
            MeshMaterial2d(custom_material.into()),
            Transform::default(),
        ));
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
