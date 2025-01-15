
use base::array2;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CellState
{
    #[default]
    None = 0,
    Mine = 1,
    NonPlayable = 2,
}

#[derive(Debug, Clone)]
struct Board
{
    states: array2::Array2<CellState>,
}

impl Board
{
    pub fn new(width: i32, height: i32) -> Self
    {
        Self
        {
            states: array2::Array2::new(width, height),
        }
    }
}

pub fn hello_sim()
{
    println!("Hello, sim!");
}
