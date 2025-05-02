
pub fn write_sync<T: serde::Serialize>(data: &T, file_path: &std::path::Path) -> std::io::Result<()>
{
    let full_path = std::path::Path::new(crate::assets::ASSETS_FOLDER).join(file_path);
    let file = std::fs::File::create(full_path)?;
    let writer = std::io::BufWriter::new(file);
    let pretty = ron::ser::PrettyConfig::default();
    ron::ser::to_writer_pretty(writer, data, pretty).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn read_sync<T: for<'de> serde::Deserialize<'de>>(file_path: &std::path::Path) -> std::io::Result<T>
{
    let full_path = std::path::Path::new(crate::assets::ASSETS_FOLDER).join(file_path);
    let file = std::fs::File::open(full_path)?;
    let reader = std::io::BufReader::new(file);
    ron::de::from_reader(reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

