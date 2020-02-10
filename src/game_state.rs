
use cgmath::*;
use ndarray::*;
use rand::Rng;
use rand::distributions::{ Uniform, Distribution, Standard, };
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum Biome {
    Desert,
    Grassland,
    Hill,
    Mountain,
    Ocean,
    Snow,
}

impl Distribution<Biome> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng:&mut R) -> Biome {
       use rand::seq::IteratorRandom; // for choose.
        Biome::iter().choose(rng).unwrap()
    }
}

impl Biome {
    pub fn color(&self) -> Vector3<f32> {
        match *self {
            Biome::Desert => Vector3::new(1.0, 1.0, 0.7),
            Biome::Grassland => Vector3::new(0.0, 1.0, 0.0),
            Biome::Hill => Vector3::new(1.0, 1.0, 0.7),
            Biome::Mountain => Vector3::new(1.0, 1.0, 1.0),
            Biome::Ocean => Vector3::new(0.0, 0.0, 1.0),
            Biome::Snow => Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Settler,
    //Soldier,
    //Scout,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub t: UnitType,
    pub loc: GridLocation,
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub next_unit: UnitType,
    pub next_unit_ready: u32,
    pub loc: GridLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GridLocation {
    pub loc: (usize, usize),
}

impl GridLocation {
    pub fn left<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (grid_dim_x, _grid_dim_y) = grid.dim();
        let new_vert = self.loc.0 as isize - distance as isize;
        GridLocation{ loc: (num::clamp(new_vert, 0, grid_dim_x as isize - 1) as usize, self.loc.1) }
    }

    pub fn right<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (grid_dim_x, _grid_dim_y) = grid.dim();
        let new_vert = self.loc.0 + distance;
        GridLocation{ loc: (num::clamp(new_vert, 0, grid_dim_x - 1), self.loc.1) }
    }

    pub fn up<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (_grid_dim_x, grid_dim_y) = grid.dim();
        let new_vert = self.loc.1 + distance;
        GridLocation{ loc: (self.loc.0, num::clamp(new_vert, 0, grid_dim_y - 1)) }
    }

    pub fn down<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (_grid_dim_x, grid_dim_y) = grid.dim();
        let new_vert = self.loc.1 as isize - distance as isize;
        GridLocation{ loc: (self.loc.0, num::clamp(new_vert, 0, grid_dim_y as isize - 1) as usize) }
    }
}

impl From<(usize, usize)> for GridLocation {
    fn from(other: (usize, usize)) -> Self {
        Self { loc: other }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eid<T> {
    pub id: u32,
    pub gen: u32,
    _phantom_data: std::marker::PhantomData<T>,
}

// NOTE: must manually impl Copy. see https://github.com/rust-lang/rust/issues/56008
impl <T: Clone> Copy for Eid<T> {}

#[derive(Debug, Clone)]
pub struct Entity<T> {
    pub gen: u32,
    pub data: Option<T>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub units: Vec<Eid<Unit>>,
    pub structures: Vec<Eid<Structure>>,
}


#[derive(Debug, Clone)]
pub struct GridCell {
    pub biome: Biome,
    pub unit: Option<Eid<Unit>>,
    pub structure: Option<Eid<Structure>>,
}

pub struct GameState {
    pub units: Vec<Entity<Unit>>,
    pub structures: Vec<Entity<Structure>>,

    pub players: Vec<Player>,

    pub grid: Array2<GridCell>,
    pub cursor: GridLocation,
    pub turn: u32,

    pub running: bool,
    pub yanked_location: Option<GridLocation>,

    pub command_text: String,
}

fn grid_fill(mut grid: &mut Array2<GridCell>, depth: u32, max_depth: u32, (x, y): (usize, usize)) {
    let cell = grid.get_mut((x, y)).unwrap();
    if cell.biome == Biome::Ocean {
        cell.biome = rand::random();
    } else {
        return ();
    }

    let (grid_dim_x, grid_dim_y) = grid.dim();
    if depth < max_depth {
        if rand::random::<bool>() {
            // trend vertically
            if y < grid_dim_y - 1 { grid_fill(&mut grid, depth + 1, max_depth, (x, y + 1)); }
            if y > 0 { grid_fill(&mut grid, depth + 1, max_depth, (x, y - 1)); }
            if x < grid_dim_x - 1 { grid_fill(&mut grid, depth + 1, max_depth, (x + 1, y)); }
            if x > 0 { grid_fill(&mut grid, depth + 1, max_depth, (x - 1, y)); }
        } else {
            // trend horizontally
            if x < grid_dim_x - 1 { grid_fill(&mut grid, depth + 1, max_depth, (x + 1, y)); }
            if x > 0 { grid_fill(&mut grid, depth + 1, max_depth, (x - 1, y)); }
            if y < grid_dim_y - 1 { grid_fill(&mut grid, depth + 1, max_depth, (x, y + 1)); }
            if y > 0 { grid_fill(&mut grid, depth + 1, max_depth, (x, y - 1)); }
        }
    }
}


impl GameState {
    pub fn new() -> Result<GameState, String> {
        let mut grid = Array2::from_shape_fn(
            (5000, 5000),
            |(_x, _y)| {
                GridCell {
                    biome: Biome::Ocean,
                    unit: None,
                    structure: None,
                }
            }
        );

        let num_continents = 10;
        let (grid_dim_x, grid_dim_y) = grid.dim();

        let mut rng = rand::thread_rng();
        let rand_x = Uniform::from(0..grid_dim_x);
        let rand_y = Uniform::from(0..grid_dim_y);

        for _ in 0..num_continents {
            let x = rand_x.sample(&mut rng);
            let y = rand_y.sample(&mut rng);
            grid_fill(&mut grid, 0, 50, (x, y));
        }

        let state = GameState {
            units: Vec::new(),
            structures: Vec::new(),

            players: Vec::new(),

            cursor: Default::default(),
            grid,
            turn: 0,

            running: true,
            yanked_location: None,

            command_text: String::new(),
        };

        Ok(state)
    }

    pub fn add_unit(&mut self, unit: Unit) -> Eid<Unit> {
        let id = self.units.len();
        let eid = Eid { id: id as u32, gen: 0, _phantom_data: std::marker::PhantomData };
        let loc = unit.loc;

        let cell = self.grid.get_mut(loc.loc).unwrap();
        assert!(cell.unit.is_none());
        cell.unit = Some(eid.clone());
        let entity = Entity {
            gen: eid.gen,
            data: Some(unit),
        };
        self.units.push(entity);
        eid
    }

    pub fn add_structure(&mut self, structure: Structure) -> Eid<Structure> {
        let id = self.structures.len();
        let eid = Eid { id: id as u32, gen: 0, _phantom_data: std::marker::PhantomData };
        let loc = structure.loc;

        let cell = self.grid.get_mut(loc.loc).unwrap();
        assert!(cell.structure.is_none());
        cell.structure = Some(eid.clone());
        let entity = Entity {
            gen: eid.gen,
            data: Some(structure),
        };
        self.structures.push(entity);
        eid
    }

    pub fn get_unit(&self, eid: Eid<Unit>) -> Option<&Unit> {
        let entity = &self.units[eid.id as usize];
        if entity.gen == eid.gen {
            entity.data.as_ref()
        } else {
            None
        }
    }

    pub fn get_structure(&self, eid: Eid<Structure>) -> Option<&Structure> {
        let entity = &self.structures[eid.id as usize];
        if entity.gen == eid.gen {
            entity.data.as_ref()
        } else {
            None
        }
    }
}


