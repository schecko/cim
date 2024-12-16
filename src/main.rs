
mod array2;
mod extents;
mod ron;
mod bevy_helper;

use array2::*;
use extents::*;
use ron::*;

use bevy::asset::{Assets, Asset};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::render::{settings::WgpuSettings, RenderPlugin, settings::Backends};
use bevy::sprite::*;
use bitflags::bitflags;

#[derive(serde::Deserialize, serde::Serialize, Asset, TypePath, Default)]
struct BoardVisTuning
{
    cell_size: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial
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

fn setup
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
)
{
    commands.spawn
    (
        Camera2d::default()
    );

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

    let error = ron::write_sync(&BoardVisTuning::default(), &std::path::Path::new("tuning/board_vis.ron"));
    println!("write result: {:?}", error);
    // TODO load directly? just instantitate instead?
    // can't load sync?
    let _board_vis_tuning = asset_server.load::<BoardVisTuning>("tuning/board_vis.ron");

    let mesh = if false
    {
        meshes.add(Rectangle::default())
    }
    else
    {
        let mut v_pos = vec!
        [
            [0.0, 0.0, 0.0], // TL
            [1.0, 0.0, 0.0], // TR
            [0.0, 1.0, 0.0], // BL
            [1.0, 1.0, 0.0], // BR
        ];
        let mut v_color: Vec<[f32; 4]> = vec![LinearRgba::WHITE.to_f32_array(); 4];
        let mut v_uv: Vec<[f32; 2]> = vec!
        [
            [0.0, 1.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0],
        ];
        let mut v_normal: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

        let mut indices = vec!
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

        meshes.add(mesh)
    };

    for pos in size.positions_row_major()
    {
        let scale = Vec3::splat(28.0);
        let translation = Vec2::new(pos.0 as f32, -(pos.1 as f32)).extend(0.0) * scale;
        commands
            .spawn
            ((
                Mesh2d(mesh.clone().into()),
                MeshMaterial2d(custom_material.clone().into()),
                Transform::default()
                    .with_translation(translation)
                    .with_scale(scale),
            ))
            .insert(VisCell{ index: size.get_index_row_major(pos.0, pos.1).unwrap(), pos });
    }
}

fn find_assets_folder() -> Result<(), std::io::Error>
{
    let mut current_dir = std::env::current_dir()?;

    while !current_dir.as_os_str().is_empty()
    {
        let assets_path = current_dir.join(crate::bevy_helper::ASSETS_FOLDER);
        if assets_path.is_dir()
        {
            std::env::set_current_dir(&current_dir)?;
            std::env::set_var(crate::bevy_helper::ASSET_ROOT_ENV, &current_dir);
            return Ok(());
        }
        current_dir = match current_dir.parent()
        {
            Some(inner) => inner.to_path_buf(),
            _ => break,
        };
    }

    Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not find assets folder"))
}

fn main()
{
    let ext = crate::extents::Extents{ width: 10, height: 10 };
    let _arr = ext.neighbours::<{ crate::extents::Neighbours::Top.bits() }>( 0, 0 );
    let _arr = ext.neighbours::<{ crate::extents::Neighbours::Top.union(crate::extents::Neighbours::Bottom).bits() }>( 0, 0 );
    let _ = find_assets_folder();

    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::DX12),
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        .add_plugins(RonAssetPlugin::<BoardVisTuning>::default())
        .add_plugins(FpsOverlayPlugin
        {
            config: FpsOverlayConfig
            {
                enabled: true,
                text_config: TextFont
                {
                    font_size: 20.0,
                    ..default()
                },
                ..default()
            },
        })
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}
