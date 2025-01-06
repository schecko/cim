
use base::array2::*;
use base::extents::*;
use base::ronx::*;
use base::tuning::Tuning;
use bevyx::ron::RonAssetPlugin;
use crate::board_vis_tuning::*;

use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::*;
use bitflags::bitflags;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GridMaterial
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

fn startup
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    vis_tuning: Res<BoardVisTuning>,
    asset_server: Res<AssetServer>,
)
{
    let size = Extents::new(5, 5);

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
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let tr = GridVert
    {
        pos: Vec2::new(0.5, -0.5),
        color: LinearRgba::BLACK,
        uv: Vec2::new(1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let bl = GridVert
    {
        pos: Vec2::new(-0.5, 0.5),
        color: LinearRgba::BLACK,
        uv: Vec2::new(0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let br = GridVert
    {
        pos: Vec2::new(0.5, 0.5),
        color: LinearRgba::BLACK,
        uv: Vec2::new(1.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };

    let mut geo = GeoBuilder::default();
    for y in 0..=size.height
    {
        for x in 0..=size.width
        {
            let cell_intersection = Vec2::new(x as f32, y as f32);
            let tli = geo.insert_vert(tl.from_pos(cell_intersection * vis_tuning.cell_size + tl.pos * vis_tuning.cell_size * 0.5));
            let tri = geo.insert_vert(tr.from_pos(cell_intersection * vis_tuning.cell_size + tr.pos * vis_tuning.cell_size * 0.5));
            let bri = geo.insert_vert(br.from_pos(cell_intersection * vis_tuning.cell_size + br.pos * vis_tuning.cell_size * 0.5));
            let bli = geo.insert_vert(bl.from_pos(cell_intersection * vis_tuning.cell_size + bl.pos * vis_tuning.cell_size * 0.5));

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
        v_pos.push(vert.pos.extend(1.0).to_array());
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
            MeshMaterial2d(custom_material.into()),
            Transform::default(),
        ));
}

pub struct GridVisPlugin;

impl Plugin for GridVisPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(Material2dPlugin::<GridMaterial>::default())
            .add_systems(Startup, startup);
    }
}
