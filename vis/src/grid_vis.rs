
use crate::board_vis_tuning::*;
use crate::layers;

use sim::grid::*;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::*;

// TODO schecko
#[allow(dead_code)]
#[derive(Debug, Clone, Component)]
pub struct EntityIndex2(IVec2);

#[derive(Debug, Clone, Component)]
pub struct EntityIndex(usize);

#[derive(Debug, Clone, Component)]
pub struct Mine;

#[derive(Debug, Clone, Component)]
pub struct Cover;

#[derive(Debug, Clone, Component)]
struct Adjacency;

#[derive(Debug, Clone, Resource)]
pub struct GridVis
{
    // TODO I dont want vis to own the sim, it should be in svc or above
    pub grid: Grid,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial
{
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for GridMaterial
{
    fn fragment_shader() -> ShaderRef
    {
        "shaders/grid.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d
    {
        AlphaMode2d::Blend
    }

    fn specialize(
        _descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
struct GridVert
{
    pos: Vec2,
    color: LinearRgba,
    uv: Vec2,
    normal: Vec3
}

impl PartialEq for GridVert
{
    fn eq(&self, other: &Self) -> bool
    {
        self.pos.eq(&other.pos)
    }

    fn ne(&self, other: &Self) -> bool
    {
        self.pos.ne(&other.pos)
    }
}

impl GridVert
{
    fn from_pos(&self, pos: Vec2) -> Self
    {
        Self
        {
            pos,
            color: self.color,
            uv: self.uv,
            normal: self.normal,
        }
    }
}

#[derive(Debug, Default)]
struct GeoBuilder<Vert>
{
    staged: Vec<Vert>,
    indices: Vec<u32>,
}

impl<Vert: PartialEq> GeoBuilder<Vert>
{
    fn insert_vert(&mut self, vert: Vert) -> u32
    {
        for (index, staged_vert) in self.staged.iter().enumerate()
        {
            if *staged_vert == vert
            {
                return index as u32;
            }
        }

        let index = self.staged.len() as u32;
        self.staged.push(vert);
        index
    }

    fn insert_index(&mut self, index: u32)
    {
        self.indices.push(index);
    }

    fn drain(self) -> (Vec<Vert>, Vec<u32>)
    {
        (self.staged, self.indices)
    }
}

pub fn spawn_grid
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    vis_tuning: Res<BoardVisTuning>,
    grid_vis: Res<GridVis>,
)
{
    let size = &grid_vis.grid.size();
    let custom_material = materials.add
    (
        GridMaterial
        {
            color: LinearRgba::BLACK.into(),
        }
    );

    let tl = GridVert
    {
        pos: Vec2::new(-0.5, -0.5),
        color: LinearRgba::BLACK,
        uv: Vec2::new(0.0, 1.0),
        normal: Vec3::new(0.5, 0.5, 1.0),
    };
    let tr = GridVert
    {
        pos: Vec2::new(0.5, -0.5),
        color: LinearRgba::BLACK,
        uv: Vec2::new(1.0, 1.0),
        normal: Vec3::new(-0.5, 0.5, 1.0),
    };
    let bl = GridVert
    {
        pos: Vec2::new(-0.5, 0.5),
        color: LinearRgba::BLACK,
        uv: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.5, -0.5, 1.0),
    };
    let br = GridVert
    {
        pos: Vec2::new(0.5, 0.5),
        color: LinearRgba::BLACK,
        uv: Vec2::new(1.0, 0.0),
        normal: Vec3::new(-0.5, -0.5, 1.0),
    };

    let mut geo = GeoBuilder::default();

    let corner_size = vis_tuning.cell_size * 0.15;

    // intersection points
    for y in 0..=size.height
    {
        for x in 0..=size.width
        {
            let cell_intersection = Vec2::new(x as f32, y as f32);
            let tli = geo.insert_vert(tl.from_pos(cell_intersection * vis_tuning.cell_size + tl.pos * corner_size));
            let tri = geo.insert_vert(tr.from_pos(cell_intersection * vis_tuning.cell_size + tr.pos * corner_size));
            let bli = geo.insert_vert(bl.from_pos(cell_intersection * vis_tuning.cell_size + bl.pos * corner_size));
            let bri = geo.insert_vert(br.from_pos(cell_intersection * vis_tuning.cell_size + br.pos * corner_size));

            geo.insert_index(tli);
            geo.insert_index(tri);
            geo.insert_index(bli);

            geo.insert_index(tri);
            geo.insert_index(bri);
            geo.insert_index(bli);
        }
    }

    // horizontal grid-aligned lines 
    for y in 0..=size.height
    {
        for x in 0..size.width
        {
            let cell_intersection = Vec2::new(x as f32, y as f32);
            let next_cell_intersection = Vec2::new((x + 1) as f32, y as f32);
            let tli = geo.insert_vert(tr.from_pos(cell_intersection * vis_tuning.cell_size + tr.pos * corner_size));
            let tri = geo.insert_vert(tl.from_pos(next_cell_intersection * vis_tuning.cell_size + tl.pos * corner_size));
            let bli = geo.insert_vert(br.from_pos(cell_intersection * vis_tuning.cell_size + br.pos * corner_size));
            let bri = geo.insert_vert(bl.from_pos(next_cell_intersection * vis_tuning.cell_size + bl.pos * corner_size));

            geo.insert_index(tli);
            geo.insert_index(tri);
            geo.insert_index(bli);

            geo.insert_index(tri);
            geo.insert_index(bri);
            geo.insert_index(bli);
        }
    }

    // vertical grid-aligned lines 
    for y in 0..size.height
    {
        for x in 0..=size.width
        {
            let cell_intersection = Vec2::new(x as f32, y as f32);
            let next_cell_intersection = Vec2::new(x as f32, (y + 1) as f32);
            let tli = geo.insert_vert(bl.from_pos(cell_intersection * vis_tuning.cell_size + bl.pos * corner_size));
            let tri = geo.insert_vert(br.from_pos(cell_intersection * vis_tuning.cell_size + br.pos * corner_size));
            let bli = geo.insert_vert(tl.from_pos(next_cell_intersection * vis_tuning.cell_size + tl.pos * corner_size));
            let bri = geo.insert_vert(tr.from_pos(next_cell_intersection * vis_tuning.cell_size + tr.pos * corner_size));

            geo.insert_index(tli);
            geo.insert_index(tri);
            geo.insert_index(bli);

            geo.insert_index(tri);
            geo.insert_index(bri);
            geo.insert_index(bli);
        }
    }

    let mut v_pos = vec![];
    let mut v_color = vec![];
    let mut v_uv = vec![];
    let mut v_normal = vec![];

    let (verts, indices) = geo.drain();
    for vert in verts
    {
        v_pos.push(vert.pos.extend(layers::GRID_LINE).to_array());
        v_color.push(vert.color.to_f32_array());
        v_uv.push(vert.uv.to_array());
        v_normal.push(vert.normal.to_array());
    }

    let mut mesh = Mesh::new
    (
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_normal);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    let mesh_id = meshes.add(mesh);

    commands
        .spawn
        ((
            Mesh2d(mesh_id.into()),
            MeshMaterial2d(custom_material.into())
        ));
}

pub fn spawn_mines
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    vis_tuning: Res<BoardVisTuning>,
    grid_vis: Res<GridVis>,
)
{
    let image = asset_server.load("textures/mine.png");

    let mine = Sprite
    {
        image,
        custom_size: Some(vis_tuning.cell_size),
        anchor: Anchor::BottomLeft,
        ..default()
    };
    for (index2, state) in grid_vis.grid.states.enumerate()
    {
        if !state.contains(CellState::Mine)
        {
            continue;
        }

        let world_pos = index2.as_vec2() * vis_tuning.cell_size;
        commands.spawn
        ((
            Mine,
            EntityIndex(grid_vis.grid.states.get_index(index2).unwrap()),
            EntityIndex2(index2),
            mine.clone(),
            Transform::from_translation(world_pos.extend(layers::MINE))
        ));
    }
}

pub fn spawn_covers
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    vis_tuning: Res<BoardVisTuning>,
    grid_vis: Res<GridVis>,
)
{
    let image = asset_server.load("textures/cover.png");

    let cover = Sprite
    {
        image,
        custom_size: Some(vis_tuning.cell_size),
        anchor: Anchor::BottomLeft,
        ..default()
    };
    for (index2, state) in grid_vis.grid.states.enumerate()
    {
        if state.contains(CellState::Revealed) || state.contains(CellState::NonPlayable)
        {
            continue;
        }

        let world_pos = index2.as_vec2() * vis_tuning.cell_size;
        commands.spawn
        ((
            Cover,
            EntityIndex(grid_vis.grid.states.get_index(index2).unwrap()),
            EntityIndex2(index2),
            cover.clone(),
            Transform::from_translation(world_pos.extend(layers::COVER))
        ));
    }
}

pub fn reveal_covers
(
    mut cover_query: Query<(&mut Visibility, &EntityIndex), With<Cover>>,
    grid_vis: ResMut<GridVis>,
)
{
    for (mut visibility, index) in &mut cover_query
    {
        let Some(state) = grid_vis.grid.states.get_by_index(index.0) else
        {
            assert!(false, "covers should always be kept up to date with the size of the grid");
            continue;
        };

        *visibility = if state.contains(CellState::Revealed)
        {
            Visibility::Hidden
        }
        else
        {
            Visibility::Visible
        };
    }
}

pub fn spawn_adjacency
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    vis_tuning: Res<BoardVisTuning>,
    grid_vis: Res<GridVis>,
)
{
    let mut it = vis_tuning.adjacency_images
        .iter()
        .map( |path| asset_server.load(&**path) );
    let images = std::array::from_fn::<Handle<Image>, 8, _>( |_| it.next().unwrap() );

    for (index, (adj, state)) in grid_vis.grid.adjacency.raw_iter().zip(grid_vis.grid.states.raw_iter()).enumerate()
    {
        if state.intersects(CellState::NonPlayable | CellState::Mine) || *adj == 0
        {
            continue;
        }

        let adj_sprite = Sprite
        {
            image: images[(adj - 1) as usize].clone(),
            custom_size: Some(vis_tuning.cell_size),
            anchor: Anchor::BottomLeft,
            ..default()
        };

        let index2 =  grid_vis.grid.states.get_index2(index).unwrap();
        let world_pos = index2.as_vec2() * vis_tuning.cell_size;
        commands.spawn
        ((
            Adjacency,
            EntityIndex2(index2),
            EntityIndex(index),
            adj_sprite,
            Transform::from_translation(world_pos.extend(layers::ADJACENCY))
        ));
    }
}

pub struct GridVisPlugin;
impl Plugin for GridVisPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(Material2dPlugin::<GridMaterial>::default())
            ;
    }
}
