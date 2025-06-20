
use crate::board_vis_tuning::*;
use crate::layers;
use crate::grid_entities::GridVis;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::*;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial
{
    #[uniform(0)]
    tint: LinearRgba,
    
    #[texture(1)]
    #[sampler(2)]
    base: Handle<Image>
}

#[derive(Component)]
pub struct GridLines;

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
    uv: Vec2,
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
            uv: self.uv,
        }
    }
}

#[derive(Debug, Default)]
struct GeoBuilder
{
    pos: Vec<Vec2>,
    uv: Vec<Vec2>,
    indices: Vec<u32>,
}

impl GeoBuilder
{
    fn insert_vert(&mut self, pos: Vec2, uv: Vec2) -> u32
    {
        let index = self.pos.len() as u32;
        self.pos.push(pos);
        self.uv.push(uv);
        index
    }

    fn insert_index(&mut self, index: u32)
    {
        self.indices.push(index);
    }

    fn drain(self) -> (Vec<Vec2>, Vec<Vec2>, Vec<u32>)
    {
        (self.pos, self.uv, self.indices)
    }
}

pub fn spawn_lines
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    asset_server: Res<AssetServer>,
    vis_tuning: Res<BoardVisTuning>,
    grid_vis: Res<GridVis>,
)
{
    let size = &grid_vis.grid.size();
    let custom_material = materials.add
    (
        GridMaterial
        {
            tint: LinearRgba::BLACK.into(),
            base: asset_server.load("textures/line.png"),
        }
    );

    let mut geo = GeoBuilder::default();
    let half_line_width = vis_tuning.cell_size * 0.15 * 0.5;

    const POS_TL: Vec2 = Vec2::new(-1.0, -1.0);
    const POS_TR: Vec2 = Vec2::new( 1.0, -1.0);
    const POS_BL: Vec2 = Vec2::new(-1.0,  1.0);
    const POS_BR: Vec2 = Vec2::new( 1.0,  1.0);
    
    const UV_TL: Vec2 = Vec2::new(0.0, 0.0);
    const UV_TR: Vec2 = Vec2::new(1.0, 0.0);
    const UV_BL: Vec2 = Vec2::new(0.0, 1.0);
    const UV_BR: Vec2 = Vec2::new(1.0, 1.0);
    
    {
        // horizontal grid-aligned lines 
        let offset = Vec2::new(0.0, half_line_width.y);
        for y in 0..=size.height
        {
            for x in 0..size.width
            {
                let cell_intersection = Vec2::new(x as f32, y as f32);
                let next_cell_intersection = Vec2::new((x + 1) as f32, y as f32);
                // UVs are rotated clockwise 90 degrees
                let tli = geo.insert_vert(cell_intersection * vis_tuning.cell_size + offset, UV_TR);
                let tri = geo.insert_vert(next_cell_intersection * vis_tuning.cell_size + offset, UV_BR);
                let bli = geo.insert_vert(cell_intersection * vis_tuning.cell_size - offset, UV_TL);
                let bri = geo.insert_vert(next_cell_intersection * vis_tuning.cell_size - offset, UV_BL);

                geo.insert_index(tli);
                geo.insert_index(tri);
                geo.insert_index(bli);

                geo.insert_index(tri);
                geo.insert_index(bri);
                geo.insert_index(bli);
            }
        }
    }

    {
        // vertical grid-aligned lines 
        let offset = Vec2::new(half_line_width.x, 0.0);
        for y in 0..size.height
        {
            for x in 0..=size.width
            {
                let cell_intersection = Vec2::new(x as f32, y as f32);
                let next_cell_intersection = Vec2::new(x as f32, (y + 1) as f32);
                let tli = geo.insert_vert(cell_intersection * vis_tuning.cell_size - offset, UV_TL);
                let tri = geo.insert_vert(cell_intersection * vis_tuning.cell_size + offset, UV_TR);
                let bli = geo.insert_vert(next_cell_intersection * vis_tuning.cell_size - offset, UV_BL);
                let bri = geo.insert_vert(next_cell_intersection * vis_tuning.cell_size + offset, UV_BR);

                geo.insert_index(tli);
                geo.insert_index(tri);
                geo.insert_index(bli);

                geo.insert_index(tri);
                geo.insert_index(bri);
                geo.insert_index(bli);
            }
        }
    }

    let mut v_pos = vec![];
    let mut v_color = vec![];
    let mut v_uv = vec![];
    let mut v_normal = vec![];

    let (poses, uvs, indices) = geo.drain();
    const NORMAL: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    for (pos, uv) in poses.iter().zip(uvs)
    {
        v_pos.push(pos.extend(layers::GRID_LINE).to_array());
        v_color.push(LinearRgba::BLACK.to_f32_array());
        v_uv.push(uv.to_array());
        v_normal.push(NORMAL.to_array());
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
            GridLines,
            Mesh2d(mesh_id.into()),
            MeshMaterial2d(custom_material.into())
        ));
}

pub fn despawn_lines
(
    mut commands: Commands,
    entities: Query<Entity, With<GridLines>>
)
{
    for entity in entities
    {
        commands.entity(entity).despawn();
    }
}


pub struct GridLinesPlugin;
impl Plugin for GridLinesPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(Material2dPlugin::<GridMaterial>::default())
            ;
    }
}
