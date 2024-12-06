use bevy::asset::{Assets, Asset};
// use bevy::color::Color;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::render::{settings::WgpuSettings, RenderPlugin, settings::Backends};
use bevy::sprite::*;

const BEVY_ASSETS_FOLDER: &str = "assets";
const BEVY_ASSET_ROOT_ENV: &str = "BEVY_ASSET_ROOT";

fn setup_camera(mut commands: Commands)
{
    commands.spawn
    (
        Camera2dBundle::default()
    );
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

fn setup_material
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
)
{
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
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_material)
        .run();
}
