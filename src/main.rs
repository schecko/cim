
mod array2;
mod extents;

use array2::*;

use bevy::asset::{Assets, Asset};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::render::{settings::WgpuSettings, RenderPlugin, settings::Backends};
use bevy::sprite::*;
use bitflags::bitflags;

const BEVY_ASSETS_FOLDER: &str = "assets";
const BEVY_ASSET_ROOT_ENV: &str = "BEVY_ASSET_ROOT";

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

    commands.insert_resource(BoardVis
                             {
                                 cell_type: Array2::<CellType>::new(10, 10),
                             });

    // Create a custom material using the texture
    let custom_material = materials.add
    (
        CustomMaterial
        {
            color: Color::WHITE.into(), // Tint color
            color_texture: asset_server.load("textures/sample.png"),
        }
    );

    // Spawn the entity with the quad and custom material
    commands.spawn(MaterialMesh2dBundle
    {
        mesh: meshes.add(Rectangle::default()).into(),
        material: custom_material,
        visibility: Visibility::Visible,
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        ..default()
    });
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
