use bevy::asset::{Assets, Asset};
// use bevy::color::Color;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::render::{settings::WgpuSettings, RenderPlugin, settings::Backends};

const BEVY_ASSETS_FOLDER: &str = "assets";
const BEVY_ASSET_ROOT_ENV: &str = "BEVY_ASSET_ROOT";

fn setup_camera(mut commands: Commands)
{
    commands.spawn
    (
        Camera2dBundle
        {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        }
    );
}

fn setup_sprite(mut commands: Commands)
{
    commands.spawn
    ((
        SpriteBundle
        {
            sprite: Sprite
            {
                color: Color::srgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(100.0, 50.0)),
                ..Default::default()
            },
            ..default()
        },
    ));
}

// #[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial
{
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl Material for CustomMaterial
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
    // Load a texture for the sprite
    let texture_handle = asset_server.load("textures/sample.png");

    // Create a quad mesh
    let quad = Mesh::from(Rectangle::new(2.0, 2.0));

    // Create a custom material using the texture
    let custom_material = materials.add
    (
        CustomMaterial
        {
            color: Color::WHITE.into(), // Tint color
            color_texture: texture_handle.clone(),
        }
    );

    // Spawn the entity with the quad and custom material
    commands.spawn(MaterialMeshBundle
    {
        mesh: meshes.add(quad),
        material: custom_material,
        ..default()
    });
}

fn find_assets_folder() -> Result<(), std::io::Error> {
    // Start with the current working directory
    let mut current_dir = std::env::current_dir()?;

    // Traverse up the directory tree
    while !current_dir.as_os_str().is_empty()
    {
        let assets_path = current_dir.join(BEVY_ASSETS_FOLDER);
        if assets_path.is_dir()
        {
            // Set the current directory to the one containing the 'assets' folder
            std::env::set_current_dir(&current_dir)?;
            std::env::set_var(BEVY_ASSET_ROOT_ENV, &current_dir);
            return Ok(());
        }
        // Move to the parent directory
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
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup_camera)
        // .add_systems(Startup, setup_sprite)
        .add_systems(Startup, setup_material)
        .run();
}
