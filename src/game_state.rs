
use cgmath::*;
use ndarray::*;
use rand::Rng;
use rand::distributions::{ Uniform, Distribution, Standard, };
use strum::IntoEnumIterator;
use rand::seq::IteratorRandom;
use rand::prelude::*;

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

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Grey,
    Lime,
    Magenta,
    Maroon,
    Navy,
    Olive,
    Purple,
    Red,
    Teal,
    White,
    Yellow,
}

impl Color {
    fn color(&self) -> Vector3<f32> {
        match *self {
            Color::Black => Vector3::new(0.1, 0.1, 0.1),
            Color::Blue => Vector3::new(0., 0., 1.),
            Color::Cyan => Vector3::new(0., 1., 1.),
            Color::Green => Vector3::new(0., 0.5, 0.),
            Color::Grey => Vector3::new(0.5, 0.5, 0.5),
            Color::Lime => Vector3::new(0., 1., 0.),
            Color::Magenta => Vector3::new(1., 0., 1.),
            Color::Maroon => Vector3::new(0.5, 0., 0.),
            Color::Navy => Vector3::new(0., 0., 0.5),
            Color::Olive => Vector3::new(0.5, 0.5, 0.),
            Color::Purple => Vector3::new(0.5, 0., 0.5),
            Color::Red => Vector3::new(1., 0., 0.),
            Color::Teal => Vector3::new(0., 0.5, 0.5),
            Color::White => Vector3::new(1., 1., 1.),
            Color::Yellow => Vector3::new(1., 1., 0.),
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

impl From<PlayerId> for usize {
    fn from(other: PlayerId) -> Self {
        other.0
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
    pub fn new(t: UnitType, player: PlayerId, loc: Vector2<isize>) -> Self {
        Unit {
            t,
            loc: GridLocation { loc },
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

impl Structure {
    pub fn new(current_time: u32, player: PlayerId, loc: Vector2<isize>) -> Self {
        Structure {
            next_unit: UnitType::Settler,
            next_unit_ready: current_time + 5,
            loc: GridLocation { loc },
            player: player.into(),
        }
    }
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

impl From<Vector2<usize>> for GridLocation {
    fn from(other: Vector2<usize>) -> Self {
        Self { loc: Vector2::new(other.x as isize, other.y as isize), }
    }
}

impl From<Vector2<isize>> for GridLocation {
    fn from(other: Vector2<isize>) -> Self {
        Self { loc: Vector2::new(other.x, other.y), }
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
pub struct UserPlayer {
    pub camera_jump: CameraJump,
    pub turn_units: Vec<Eid<Unit>>,
    pub turn_structures: Vec<Eid<Structure>>,
}

#[derive(Debug, Clone)]
pub struct AiPlayer {
}

#[derive(Debug, Clone)]
pub enum PlayerType {
    User(UserPlayer),
    Ai(AiPlayer),
}

#[derive(Debug, Clone)]
pub struct Player {
    pub units: Vec<Eid<Unit>>,
    pub structures: Vec<Eid<Structure>>,
    pub color: Vector3<f32>,

    pub player_type: PlayerType,
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

        let continent_seeds: Vec<_> = (0..num_continents).map(|_| {
                let x = rand_x.sample(&mut rng);
                let y = rand_y.sample(&mut rng);
                grid_fill(&mut grid, 0, 50, (x, y));
                Vector2::new(x, y)
            })
            .collect();

        let mut state = GameState {
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

        let mut color_iter = Color::iter();
        let user = Player {
            units: Vec::new(),
            structures: Vec::new(),
            color: color_iter.next().unwrap().color(),

            player_type: PlayerType::User(UserPlayer {
                camera_jump: CameraJump::Unit(0),
                turn_units: Vec::new(),
                turn_structures: Vec::new(),
            }),
        };
        state.players.push(user);

        // TODO start locations may overlap, causing a panic
        let num_ai = 1;
        (0..num_ai).for_each(|_i| {
            let ai = Player {
                units: Vec::new(),
                structures: Vec::new(),
                color: color_iter.next().unwrap().color(),

                player_type: PlayerType::Ai(AiPlayer{}),
            };
            state.players.push(ai);
        });

        state.players.clone().iter().enumerate().for_each(|(id, _player)| {
            let start_seed = continent_seeds.choose(&mut rng).unwrap();
            let direction: Vector2<isize> = match rand::random::<usize>() % 4 {
                0 => Vector2::new(1, 0),
                1 => Vector2::new(-1, 0),
                2 => Vector2::new(0, 1),
                3 => Vector2::new(0, -1),
                _ => panic!("only 4 directions are possible"),
            };

            let mut cell_location = start_seed.cast::<isize>().unwrap();
            loop {
                let new_loc = cell_location + direction;
                if !state.grid_contains(new_loc) {
                    break;
                }
                let grid_cell = state.get_grid(new_loc.into());
                if grid_cell.biome == Biome::Ocean || grid_cell.biome == Biome::Mountain {
                    let unit = Unit::new(UnitType::Settler, id.into(), (cell_location).cast().unwrap());
                    state.add_unit(unit);
                    break;
                } else {
                    cell_location += direction;
                }
            }

        });
        state.reset_turn();


        Ok(state)
    }

    pub fn grid_contains(&self, loc: Vector2<isize>) -> bool {
        let (grid_dim_x, grid_dim_y) = self.grid.dim();
        if loc.x >= 0 && loc.x < grid_dim_x as isize && loc.y >= 0 && loc.y < grid_dim_y as isize {
            true
        } else {
            false
        }
    }

    pub fn validate_state(&self) {
        self.grid.indexed_iter().for_each(|((x, y), cell)| {
            if let Some(u) = cell.unit {
                if let Some(unit) = self.get_unit(u) {
                    assert!(unit.loc.loc == Vector2::new(x as isize, y as isize));
                }
            }

            if let Some(u) = cell.structure {
                if let Some(s) = self.get_structure(u) {
                    assert!(s.loc.loc == Vector2::new(x as isize, y as isize));
                }
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

    pub fn check_turn_complete(&mut self, allow_unmoved_objects: bool) {
        let player = &mut self.players[self.current_player];

        let done = match &player.player_type {
            PlayerType::User(u) => (allow_unmoved_objects && self.current_player == 0) || (u.turn_units.len() == 0 && u.turn_structures.len() == 0),
            PlayerType::Ai(_ai) => true,
        };

        if done {
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
            match &mut player.player_type {
                PlayerType::User(u) => {
                    u.turn_units = player.units.clone();
                    u.turn_structures = player.structures.clone();
                    u.camera_jump = CameraJump::Unit(0);
                },
                PlayerType::Ai(_ai) => {
                },
            }
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

    pub fn delete_unit(&mut self, eid: Eid<Unit>) {
        let entity = &mut self.units[eid.id as usize];
        let opt_loc = if entity.gen == eid.gen {
            let opt_loc = if let Some(unit) = &mut entity.data {
                Some(unit.loc)
            } else {
                None
            };
            entity.data = None;
            opt_loc
        } else {
            None
        };
        if let Some(loc) = opt_loc {
            self.get_grid_mut(loc).unit = None;
        }
    }

    pub fn delete_structure(&mut self, eid: Eid<Structure>) {
        let entity = &mut self.structures[eid.id as usize];
        let opt_loc = if entity.gen == eid.gen {
            let opt_loc = if let Some(s) = &mut entity.data {
                Some(s.loc)
            } else {
                None
            };
            entity.data = None;
            opt_loc
        } else {
            None
        };
        if let Some(loc) = opt_loc {
            self.get_grid_mut(loc).structure = None;
        }
    }
}


