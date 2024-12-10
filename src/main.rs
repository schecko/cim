
mod array2;
mod extents;

use array2::*;
use extents::*;

use bevy::asset::{Assets, Asset};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::render::{settings::WgpuSettings, RenderPlugin, settings::Backends};
use bevy::sprite::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bitflags::bitflags;

const BEVY_ASSETS_FOLDER: &str = "assets";
const BEVY_ASSET_ROOT_ENV: &str = "BEVY_ASSET_ROOT";

#[derive(serde::Deserialize, Asset, TypePath)]
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
        Camera2dBundle::default()
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

    // TODO load directly? just instantitate instead?
    // can't load sync?
    let _board_vis_tuning = asset_server.load::<BoardVisTuning>("tuning/board_vis.json");

    let mesh = meshes.add(Rectangle::default());

    for pos in size.positions_row_major()
    {
        let scale = Vec3::splat(28.0);
        let translation = Vec2::new(pos.0 as f32, -(pos.1 as f32)).extend(0.0) * scale;
        commands
            .spawn
            (
                MaterialMesh2dBundle
                {
                    mesh: mesh.clone().into(),
                    material: custom_material.clone(),
                    visibility: Visibility::Visible,
                    transform: Transform::default()
                        .with_translation(translation)
                        .with_scale(scale),
                    ..default()
                } 
            )
            .insert(VisCell{ index: size.get_index_row_major(pos.0, pos.1).unwrap(), pos });
    }
}

fn find_assets_folder() -> Result<(), std::io::Error>
{
    let mut current_dir = std::env::current_dir()?;

    while !current_dir.as_os_str().is_empty()
    {
        let assets_path = current_dir.join(BEVY_ASSETS_FOLDER);
        if assets_path.is_dir()
        {
            std::env::set_current_dir(&current_dir)?;
            std::env::set_var(BEVY_ASSET_ROOT_ENV, &current_dir);
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
        .add_plugins(JsonAssetPlugin::<BoardVisTuning>::new(&["json"]))
        .add_plugins(FpsOverlayPlugin
        {
            config: FpsOverlayConfig
            {
                text_config: TextStyle
                {
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    font: default(),
                },
            },
        })
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}
