
use base::extents::*;
use base::array2::*;
use ron::*;

use bevy::asset::{Assets, Asset};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::render::{settings::WgpuSettings, RenderPlugin, settings::Backends};
use bevy::sprite::*;
use bitflags::bitflags;

fn find_assets_folder() -> Result<(), std::io::Error>
{
    let mut current_dir = std::env::current_dir()?;

    while !current_dir.as_os_str().is_empty()
    {
        let assets_path = current_dir.join(bevyx::helper::ASSETS_FOLDER);
        if assets_path.is_dir()
        {
            std::env::set_current_dir(&current_dir)?;
            std::env::set_var(bevyx::helper::ASSET_ROOT_ENV, &current_dir);
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
    let ext = base::extents::Extents{ width: 10, height: 10 };
    let _arr = ext.neighbours::<{ base::extents::Neighbours::Top.bits() }>( 0, 0 );
    let _arr = ext.neighbours::<{ base::extents::Neighbours::Top.union(base::extents::Neighbours::Bottom).bits() }>( 0, 0 );
    let _ = find_assets_folder();

    base::hello_base();
    bevyx::hello_bevyx();
    sim::hello_sim();
    vis::hello_vis();

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
                enabled: true,
                text_config: TextFont
                {
                    font_size: 20.0,
                    ..default()
                },
                ..default()
            },
        })
        .add_plugins(vis::GameVisPlugin)
        .run();
}
