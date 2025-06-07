
use base::array2::Array2;
use base::extents;

use bitflags::bitflags;

bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CellState: u8
    {
        const None = 0 << 0;
        const Mine = 1 << 0;
        const Revealed = 1 << 1;
        const NonPlayable = 1 << 2;
        const Flag = 1 << 3;
    }
}

#[derive(Debug, Clone)]
pub struct Grid
{
    pub first_guess: bool,
    pub states: Array2<CellState>,
    pub adjacency: Array2<u8>,
}

impl Grid
{
    pub fn new(width: i32, height: i32) -> Self
    {
        Self
        {
            first_guess: true,
            states: Array2::new(width, height),
            adjacency: Array2::new(width, height),
        }
    }

    pub fn from_size(size: extents::Extents) -> Self
    {
        Self
        {
            first_guess: true,
            states: Array2::from_size(size),
            adjacency: Array2::from_size(size),
        }
    }

    pub fn size(&self) -> extents::Extents
    {
        self.states.size()
    }

    pub fn clear(&mut self)
    {
        self.states.fill_with(CellState::None);
        self.adjacency.fill_with(0);
    }

    pub fn update_adjacency(&mut self)
    {
        let size = self.states.size();
        for pos in size.index2_space()
        {
            let mut adj = 0;
            for neighbour_pos in size.neighbours::<{ base::extents::Neighbours::All.bits() }>(pos)
            {
                let state = self.states.get_by_index2(neighbour_pos).unwrap();
                if state.intersects(CellState::Mine)
                {
                    adj += 1;
                }
            }
            assert!(adj <= 8);
            self.adjacency.set_by_index2(pos, adj).unwrap();
        }
    }
}

