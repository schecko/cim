use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, LoadContext};
use ron::de::from_bytes;
use std::marker::PhantomData;
use thiserror::Error;

pub fn write_sync<T: serde::Serialize>(data: &T, file_path: &std::path::Path) -> std::io::Result<()>
{
    let full_path = std::path::Path::new(crate::bevy_helper::ASSETS_FOLDER).join(file_path);
    let file = std::fs::File::create(full_path)?;
    let writer = std::io::BufWriter::new(file);
    let pretty = ron::ser::PrettyConfig::default();
    ron::ser::to_writer_pretty(writer, data, pretty).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn read_sync<T: for<'de> serde::Deserialize<'de>>(file_path: &std::path::Path) -> std::io::Result<T>
{
    let full_path = std::path::Path::new(crate::bevy_helper::ASSETS_FOLDER).join(file_path);
    let file = std::fs::File::open(full_path)?;
    let reader = std::io::BufReader::new(file);
    ron::de::from_reader(reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}


pub struct RonAssetPlugin<A>
{
    extensions: &'static [&'static str],
    _marker: PhantomData<A>,
}

impl<A> Plugin for RonAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App)
    {
        app.init_asset::<A>()
            .register_asset_loader(RonAssetLoader::<A>
            {
                extensions: self.extensions,
                _marker: PhantomData,
            });
    }
}

impl<A> Default for RonAssetPlugin<A>
{
    fn default() -> Self
    {
        Self
        {
            extensions: &["ron"],
            _marker: PhantomData
        }
    }
}

impl<A> RonAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    pub fn new(extensions: &'static [&'static str]) -> Self
    {
        Self {
            extensions,
            _marker: PhantomData,
        }
    }
}

pub struct RonAssetLoader<A>
{
    extensions: &'static [&'static str],
    _marker: PhantomData<A>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RonLoaderError
{
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonError(#[from] ron::error::SpannedError),
}

impl<A> AssetLoader for RonAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = RonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error>
    {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = from_bytes::<A>(&bytes)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str]
    {
        &self.extensions
    }
}
