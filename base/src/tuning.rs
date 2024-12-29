
pub trait Tuning
{
    fn path() -> &'static std::path::Path;

    // TODO easy load?
    /*
    fn load() -> Self
    {
        let tuning = match bevyx::ron::read_sync(&std::path::Path::new("tuning/board_vis.ron"))
        {
            Ok(tuning) =>
            {
                tuning
            },
            Err(err) =>
            {
                eprintln!("vis::pre_startup -- Failed to load board_vis_tuning");
                debug_assert!(false, "Failed to load board_vis_tuning");
                BoardVisTuning::default()
            }
        };
    }
    */
}
