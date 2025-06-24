
use bevy::asset::LoadedFolder;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;

use base::assets::find_folder;
use base::assets::ASSETS_FOLDER;
use base::assets::ROOT_FOLDER;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState
{
    #[default]
    LoadAssets,
    Finish,
}

#[derive(Component, Default)]
struct AtlasFolder(Handle<LoadedFolder>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(AtlasFolder(asset_server.load_folder("textures/gameplay/")));
}

fn check_assets
(
    mut next_state: ResMut<NextState<AppState>>,
    atlas_folders: Query<&AtlasFolder>,
    asset_server: Res<AssetServer>
)
{
    let mut loaded = 0;
    let mut total = 0;
    // NOTE: this is at best O(n^2), but is probably worse
    for atlas_folder in atlas_folders
    {
        total += 1;
        if asset_server.is_loaded_with_dependencies(&atlas_folder.0)
        {
            loaded += 1;
        }
    }

    if loaded == total
    {
        println!("Assets Loaded, switching to processing");
        next_state.set(AppState::Finish);
    }
}

fn do_exit
(
    mut exit: EventWriter<AppExit>,
)
{
    println!("Complete");
    exit.write(AppExit::Success);
}

fn process_atlases
(
    atlas_folders: Query<&AtlasFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
)
{
    for atlas_folder in atlas_folders
    {
        let Some(folder) = loaded_folders.get(&atlas_folder.0) else
        {
            continue;
        };

        let (_texture_atlas_linear, _linear_sources, _linear_texture) = create_texture_atlas(
            folder,
            None,
            Some(ImageSampler::linear()),
            &mut textures,
        );
    }
}

fn create_texture_atlas
(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>)
{
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    texture_atlas_builder.padding(padding.unwrap_or_default());
    for handle in folder.handles.iter()
    {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else
        {
            warn!(
                "{} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture_atlas_sources, texture) =
        texture_atlas_builder.build().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture_atlas_sources, texture)
}

fn main()
{
    let assets_folder = find_folder(ASSETS_FOLDER).expect("Failed to find ASSETS_FOLDER");
    let root_folder = find_folder(ROOT_FOLDER).expect("Failed to find ROOT_FOLDER");
    println!("assets folder located at {}", assets_folder.display());
    println!("root folder located at {}", root_folder.display());

    base::hello_base();
    bevyx::hello_bevyx();

    App::new()
        .add_plugins
        (
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin
                {
                    mode: AssetMode::Unprocessed,
                    file_path: root_folder.display().to_string(),
                    processed_file_path: assets_folder.display().to_string(),
                    ..default()
                })
                .set(RenderPlugin
                {
                    render_creation: WgpuSettings
                    {
                        backends: None,
                        ..default()
                    }.into(),
                    ..default()
                })
                .set(WindowPlugin
                 {
                     exit_condition: bevy::window::ExitCondition::DontExit,
                     primary_window: None,
                     ..default()
                 })
        )
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::LoadAssets), load_assets)
        .add_systems(Update, check_assets.run_if(in_state(AppState::LoadAssets)))
        .add_systems(OnEnter(AppState::Finish), process_atlases)
        .add_systems(Update, do_exit.run_if(in_state(AppState::Finish)))
        .run();
}

