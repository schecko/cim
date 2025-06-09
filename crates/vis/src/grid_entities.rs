
use crate::board_vis_tuning::*;
use crate::layers;

use base::point::Point;
use base::array2::Array2;
use base::extents::Extents;
use sim::grid::*;

use bevy::prelude::*;
use bevy::sprite::*;

use std::marker::PhantomData;

#[derive(Debug, Clone, Resource)]
pub struct EntityGrid<Tag>
{
    known: Array2<Option<Entity>>,
    _tag: PhantomData<Tag>,
}

impl<Tag> EntityGrid<Tag>
{
    fn new(size: Extents) -> Self
    {
        Self
        {
            known: Array2::from_size(size),
            _tag: PhantomData::<Tag>,
        }
    }
}

pub trait VisTag
{
    fn include_state() -> CellState;
    fn exclude_state() -> CellState;
    
    fn should_show(state: &CellState) -> bool
    {
        let include_state = Self::include_state();
        let exclude_state = Self::exclude_state();
        let include = ( *state & include_state ) == include_state;
        let exclude = ( *state & exclude_state ) == CellState::None;
        include && exclude
    }

    // TODO just spawn the whole bloody entity?
    fn layer() -> f32;
    fn sprite(vis_tuning: &BoardVisTuning, handles: &VisHandles) -> Sprite;
}

// TODO schecko
#[allow(dead_code)]
#[derive(Debug, Clone, Component)]
pub struct EntityIndex2(Point);

// TODO schecko
#[allow(dead_code)]
#[derive(Debug, Clone, Component)]
pub struct EntityIndex(usize);

#[derive(Debug, Clone, Component, Default)]
pub struct Mine;

impl VisTag for Mine
{
    fn include_state() -> CellState { CellState::Mine | CellState::Revealed }
    fn exclude_state() -> CellState { CellState::None }
    fn layer() -> f32 { layers::MINE }

    fn sprite(vis_tuning: &BoardVisTuning, handles: &VisHandles) -> Sprite
    {
        Sprite
        {
            image: handles.mine.clone(),
            custom_size: Some(vis_tuning.cell_size),
            anchor: Anchor::BottomLeft,
            ..default()
        }
    }
}

#[derive(Debug, Clone, Component, Default)]
pub struct Cover;

impl VisTag for Cover
{
    fn include_state() -> CellState { CellState::None }
    fn exclude_state() -> CellState { CellState::NonPlayable | CellState::Revealed }
    fn layer() -> f32 { layers::COVER }

    fn sprite(vis_tuning: &BoardVisTuning, handles: &VisHandles) -> Sprite
    {
        Sprite
        {
            image: handles.cover.clone(),
            custom_size: Some(vis_tuning.cell_size),
            anchor: Anchor::BottomLeft,
            ..default()
        }
    }
}

#[derive(Debug, Clone, Component, Default)]
pub struct Flag;

impl VisTag for Flag
{
    fn include_state() -> CellState { CellState::Flag }
    fn exclude_state() -> CellState { CellState::None }
    fn layer() -> f32 { layers::FLAG }

    fn sprite(vis_tuning: &BoardVisTuning, handles: &VisHandles) -> Sprite
    {
        Sprite
        {
            image: handles.flag.clone(),
            custom_size: Some(vis_tuning.cell_size),
            anchor: Anchor::BottomLeft,
            ..default()
        }
    }
}

#[derive(Debug, Clone, Component, Default)]
pub struct Adjacency;

impl VisTag for Adjacency
{
    fn include_state() -> CellState { CellState::Revealed }
    fn exclude_state() -> CellState { CellState::NonPlayable | CellState::Mine }
    fn layer() -> f32 { layers::ADJACENCY }

    fn sprite(_vis_tuning: &BoardVisTuning, _handles: &VisHandles) -> Sprite
    {
        panic!("Manually handled, maybe split into another trait?");
    }
}

#[derive(Debug, Clone, Resource)]
pub struct GridVis
{
    // TODO I dont want vis to own the sim, it should be in svc or above
    pub grid: Grid,
}

#[derive(Debug, Clone, Resource)]
pub struct VisHandles
{
    mine: Handle<Image>,
    flag: Handle<Image>,
    cover: Handle<Image>,
    adjacency: [Handle<Image>; 8],
}

pub fn init_handles
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    vis_tuning: Res<BoardVisTuning>,
)
{
    let mut adj_it = vis_tuning.adjacency_images
        .iter()
        .map( |path| asset_server.load(&**path) );
    let adjacency = std::array::from_fn::<Handle<Image>, 8, _>( |_| adj_it.next().unwrap() );

    commands.insert_resource
    (
        VisHandles
        {
            mine: asset_server.load("textures/mine.png"),
            flag: asset_server.load("textures/flag.png"),
            cover: asset_server.load("textures/cover.png"),
            adjacency,
        }
    );
}

pub fn init_known
(
    mut commands: Commands,
    grid_vis: Res<GridVis>,
)
{
    let size = grid_vis.grid.states.size();
    commands.insert_resource
    (
        EntityGrid::<Mine>::new(size),
    );
    commands.insert_resource
    (
        EntityGrid::<Flag>::new(size),
    );
    commands.insert_resource
    (
        EntityGrid::<Cover>::new(size),
    );
    commands.insert_resource
    (
        EntityGrid::<Adjacency>::new(size),
    );
}

pub fn sync_grid_entities<Tag>
(
    mut commands: Commands,
    vis_tuning: Res<BoardVisTuning>,
    grid_vis: Res<GridVis>,
    mut entity_grid: ResMut<EntityGrid<Tag>>,
    handles: Res<VisHandles>,
)
    where Tag: Component + Default + VisTag
{
    let size = grid_vis.grid.states.size();
    for index2 in size.index2_space()
    {
        let state = grid_vis.grid.states.get_by_index2(index2).unwrap();
        let vis = &mut entity_grid.known.get_by_index2_mut(index2).unwrap();

        if Tag::should_show(state) == vis.is_some()
        {
            continue;
        }

        if let Some(entity) = vis.take()
        {
            commands.entity(entity).despawn();
            continue;
        }

        let world_pos = index2.as_vec2() * vis_tuning.cell_size;
        let id = commands.spawn
        ((
            Tag::default(),
            EntityIndex(size.get_index(index2).unwrap()),
            EntityIndex2(index2),
            Tag::sprite(&vis_tuning, &handles),
            Transform::from_translation(world_pos.extend(Tag::layer()))
        )).id();
        **vis = Some(id);
    }
}

pub fn sync_adjacency
(
    mut commands: Commands,
    vis_tuning: Res<BoardVisTuning>,
    grid_vis: Res<GridVis>,
    mut entity_grid: ResMut<EntityGrid<Adjacency>>,
    handles: Res<VisHandles>,
)
{
    type Tag = Adjacency;
    
    let size = grid_vis.grid.states.size();
    for index2 in size.index2_space()
    {
        let adj = grid_vis.grid.adjacency.get_by_index2(index2).unwrap();
        let state = grid_vis.grid.states.get_by_index2(index2).unwrap();
        let vis = &mut entity_grid.known.get_by_index2_mut(index2).unwrap();

        if ( Tag::should_show(state) && *adj != 0 ) == vis.is_some()
        {
            continue;
        }

        if let Some(entity) = vis.take()
        {
            commands.entity(entity).despawn();
            continue;
        }
        
        let adj_sprite = Sprite
        {
            image: handles.adjacency[(adj - 1) as usize].clone(),
            custom_size: Some(vis_tuning.cell_size),
            anchor: Anchor::BottomLeft,
            ..default()
        };

        let index = grid_vis.grid.states.get_index(index2).unwrap();
        let world_pos = index2.as_vec2() * vis_tuning.cell_size;
        let id = commands.spawn
        ((
            Tag::default(),
            EntityIndex(index),
            EntityIndex2(index2),
            adj_sprite,
            Transform::from_translation(world_pos.extend(Tag::layer()))
        )).id();
        **vis = Some(id);
    }
}

pub fn destroy_known
(
    mut commands: Commands,
    mines: Query<Entity, With<Mine>>,
    flags: Query<Entity, With<Flag>>,
    covers: Query<Entity, With<Cover>>,
    adjacency: Query<Entity, With<Adjacency>>
)
{
    for entity in mines
    {
        commands.entity(entity).despawn();
    }
    for entity in flags
    {
        commands.entity(entity).despawn();
    }
    for entity in covers
    {
        commands.entity(entity).despawn();
    }
    for entity in adjacency
    {
        commands.entity(entity).despawn();
    }
}

pub struct GridEntitiesPlugin;
impl Plugin for GridEntitiesPlugin
{
    fn build(&self, _app: &mut App)
    {
    }
}
