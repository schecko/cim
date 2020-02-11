
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

impl UnitType {
    fn moves(self) -> usize {
        match self {
            UnitType::Settler => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(usize);

impl From<usize> for PlayerId {
    fn from(other: usize) -> Self {
        Self(other)
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub t: UnitType,
    pub loc: GridLocation,
    pub player: PlayerId,
    pub moves_remaining: usize,
}

impl Unit {
    pub fn new(t: UnitType, player: PlayerId) -> Self {
        Unit {
            t,
            loc: (0, 0).into(),
            player: player.into(),
            moves_remaining: UnitType::moves(t),
        }
    }
}

impl From<&Structure> for Unit {
    fn from(s: &Structure) -> Self {
        Unit {
            t: s.next_unit,
            loc: s.loc,
            player: s.player,
            moves_remaining: UnitType::moves(s.next_unit),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub next_unit: UnitType,
    pub next_unit_ready: u32,
    pub loc: GridLocation,
    pub player: PlayerId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridLocation {
    pub loc: Vector2<isize>,
}

impl Default for GridLocation {
    fn default() -> Self {
        GridLocation {
            loc: Vector2::new(0, 0),
        }
    }
}

impl From<GridLocation> for (usize, usize) {
    fn from(loc: GridLocation) -> (usize, usize) {
        (loc.loc.x as usize, loc.loc.y as usize)
    }
}

impl From<(usize, usize)> for GridLocation {
    fn from(other: (usize, usize)) -> Self {
        Self { loc: Vector2::new(other.0 as isize, other.1 as isize), }
    }
}

impl GridLocation {
    pub fn translate<T>(&self, grid: &Array2<T>, desired_distance: Vector2<isize>) -> (Self, Vector2<isize>) {
        let (grid_dim_x, grid_dim_y) = grid.dim();
        let new_pos = self.loc + desired_distance;
        let clamped = Vector2::new(num::clamp(new_pos.x, 0, grid_dim_x as isize - 1), num::clamp(new_pos.y, 0, grid_dim_y as isize - 1));
        let actual_distance = desired_distance - (new_pos - clamped);
        let loc = GridLocation { loc: clamped };
        (loc, actual_distance)
    }

    pub fn left<T>(&self, grid: &Array2<T>, distance: usize) -> (Self, Vector2<isize>) {
        self.translate(grid, Vector2::new(-(distance as isize), 0))
    }

    pub fn right<T>(&self, grid: &Array2<T>, distance: usize) -> (Self, Vector2<isize>) {
        self.translate(grid, Vector2::new(distance as isize, 0))
    }

    pub fn up<T>(&self, grid: &Array2<T>, distance: usize) -> (Self, Vector2<isize>) {
        self.translate(grid, Vector2::new(0, distance as isize))
    }

    pub fn down<T>(&self, grid: &Array2<T>, distance: usize) -> (Self, Vector2<isize>) {
        self.translate(grid, Vector2::new(0, -(distance as isize)))
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Eid<T> {
    pub id: u32,
    pub gen: u32,
    _phantom_data: std::marker::PhantomData<T>,
}

impl <T> PartialEq for Eid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.gen == other.gen
    }
}

// NOTE: must manually impl Copy. see https://github.com/rust-lang/rust/issues/56008
impl <T: Clone> Copy for Eid<T> {}

#[derive(Debug, Clone)]
pub struct Entity<T> {
    pub gen: u32,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Copy)]
pub enum CameraJump {
    Unit(usize),
    Structure(usize),
}

#[derive(Debug, Clone)]
pub struct Player {
    pub units: Vec<Eid<Unit>>,
    pub structures: Vec<Eid<Structure>>,

    pub camera_jump: CameraJump,
    pub turn_units: Vec<Eid<Unit>>,
    pub turn_structures: Vec<Eid<Structure>>,
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
    pub current_player: usize,

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
            (50, 50),
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
            current_player: 0,

            cursor: Default::default(),
            grid,
            turn: 0,

            running: true,
            yanked_location: None,

            command_text: String::new(),
        };

        Ok(state)
    }

    pub fn validate_state(&self) {
        self.grid.indexed_iter().for_each(|((x, y), cell)| {
            if let Some(u) = cell.unit {
                let unit = self.get_unit(u).unwrap();
                assert!(unit.loc.loc == Vector2::new(x as isize, y as isize));
            }

            if let Some(u) = cell.structure {
                let s = self.get_structure(u).unwrap();
                assert!(s.loc.loc == Vector2::new(x as isize, y as isize));
            }
        });

        self.units.iter().enumerate().for_each(|(i, entity)| {
            let eid = Eid {
                gen: entity.gen,
                id: i as u32,
                _phantom_data: std::marker::PhantomData,
            };
            if let Some(u) = &entity.data {
                if let Some(uid) = self.get_grid(u.loc).unit {
                    assert!(uid == eid);
                }
            }
        });

        self.structures.iter().enumerate().for_each(|(i, entity)| {
            let eid = Eid {
                gen: entity.gen,
                id: i as u32,
                _phantom_data: std::marker::PhantomData,
            };
            if let Some(s) = &entity.data {
                if let Some(sid) = self.get_grid(s.loc).structure {
                    assert!(sid == eid);
                }
            }
        });
    }

    pub fn check_turn_complete(&mut self, force_end_turn: bool) {
        let player = &mut self.players[self.current_player];

        if (force_end_turn && self.current_player == 0) || (player.turn_units.len() == 0 && player.turn_structures.len() == 0) {
            self.current_player += 1;
            if self.current_player >= self.players.len() {
                self.turn += 1;
                self.reset_turn();
                self.current_player = 0;
            }
        }
    }

    pub fn reset_turn(&mut self) {
        self.players.iter_mut().for_each(|player| {
            player.turn_units = player.units.clone();
            player.turn_structures = player.structures.clone();
            player.camera_jump = CameraJump::Unit(0);
        });

        self.units.iter_mut().for_each(|unit| {
            if let Some(u) = &mut unit.data {
                u.moves_remaining = UnitType::moves(u.t);
            }
        });
    }

    pub fn get_grid(&self, loc: GridLocation) -> &GridCell {
        self.grid.get::<(usize, usize)>(loc.into()).unwrap()
    }

    pub fn get_grid_mut(&mut self, loc: GridLocation) -> &mut GridCell {
        self.grid.get_mut::<(usize, usize)>(loc.into()).unwrap()
    }

    pub fn add_unit(&mut self, unit: Unit) -> Eid<Unit> {
        let id = self.units.len();
        let eid = Eid { id: id as u32, gen: 0, _phantom_data: std::marker::PhantomData };
        let loc = unit.loc;

        let cell = self.get_grid_mut(loc);
        assert!(cell.unit.is_none());
        cell.unit = Some(eid.clone());
        self.players[unit.player.0].units.push(eid.clone());
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

        let cell = self.get_grid_mut(loc);
        assert!(cell.structure.is_none());
        cell.structure = Some(eid.clone());
        self.players[structure.player.0].structures.push(eid.clone());

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

    pub fn get_unit_mut(&mut self, eid: Eid<Unit>) -> Option<&mut Unit> {
        let entity = &mut self.units[eid.id as usize];
        if entity.gen == eid.gen {
            entity.data.as_mut()
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

    pub fn get_structure_mut(&mut self, eid: Eid<Structure>) -> Option<&mut Structure> {
        let entity = &mut self.structures[eid.id as usize];
        if entity.gen == eid.gen {
            entity.data.as_mut()
        } else {
            None
        }
    }
}

