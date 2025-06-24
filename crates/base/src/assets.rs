
pub const ASSETS_FOLDER: &str = "assets";
pub const ROOT_FOLDER: &str = "root";

pub fn find_folder(folder: &'static str) -> Result<std::path::PathBuf, std::io::Error>
{
    let mut current_dir = std::env::current_dir()?;
    println!("startup current_dir: {:#?}", current_dir);

    while !current_dir.as_os_str().is_empty()
    {
        let assets_path = current_dir.join(folder);
        if assets_path.is_dir()
        {
            std::env::set_current_dir(&current_dir)?;
            return Ok(assets_path);
        }
        current_dir = match current_dir.parent()
        {
            Some(inner) => inner.to_path_buf(),
            _ => break,
        };
    }

    Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not find assets folder"))
}

