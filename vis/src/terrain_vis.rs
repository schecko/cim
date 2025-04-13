
use base::array2::*;
use base::extents::*;
use crate::board_vis_tuning::*;
use crate::layers;
use crate::terrain_grid::CellType;
use crate::terrain_grid::TerrainGrid;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::*;

fn blur<T, const N: usize>(data: &mut Array2<T>, kernel: &[T; N], passes: u32)
    where T: Default + Copy + std::ops::AddAssign + std::ops::Mul<Output = T>
{
    assert!(N & 1 != 0); // N must be odd

    let size = data.size();
    let mut temp = Array2::<T>::from_size(size);
    let delta = N as i32 / 2;

    for _ in 0..passes
    {
        // hori pass
        for pos in data.index2_space()
        {
            let mut acc = T::default();
            for d in -delta..=delta
            {
                if let Some(&read) = data.get_by_index2(pos + Point::new(d, 0))
                {
                    acc += read * kernel[(delta + d) as usize];
                }
            }
            temp[pos] = acc;
        }

        // vert pass
        for pos in data.index2_space()
        {
            let mut acc = T::default();
            for d in -delta..=delta
            {
                if let Some(&read) = temp.get_by_index2(pos + Point::new(0, d))
                {
                    acc += read * kernel[(delta + d) as usize];
                }
            }
            data[pos] = acc;
        }
    }
}

fn guassian_blur(data: &mut Array2<f32>, passes: u32)
{
    // blur(data, &[0.25, 0.5, 0.25], passes)
    // blur(data, &[0.27406862, 0.45186276, 0.27406862], passes)
    blur(data, &[0.06136, 0.24477, 0.38774, 0.24477, 0.06136], passes)
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TerrainMaterial
{
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1, sample_type="float")]
    #[sampler(2, sampler_type="filtering")]
    elevation: Handle<Image>,

    // should be 1d, but bevy can't load png as 1d
    #[texture(3, sample_type="float", dimension="2d")]
    #[sampler(4, sampler_type="filtering")]
    color_palette: Handle<Image>,
}

impl Material2d for TerrainMaterial
{
    fn fragment_shader() -> ShaderRef
    {
        "shaders/terrain.wgsl".into()
    }

    fn specialize(
        _descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError>
    {
        // descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

pub fn startup
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    mut images: ResMut<Assets<Image>>,
    terrain_grid: Res<TerrainGrid>,
    vis_tuning: Res<BoardVisTuning>,
    asset_server: Res<AssetServer>,
)
{
    let size = terrain_grid.size();

    let elevation_handle = {
        const HEIGHT_MAP_SCALE: i32 = 4;
        let mut height_map = Array2::<f32>::new
        (
            size.width * HEIGHT_MAP_SCALE,
            size.height * HEIGHT_MAP_SCALE
        );

        for pos in height_map.index2_space()
        {
            height_map[pos] = if terrain_grid.grid[pos / HEIGHT_MAP_SCALE] == CellType::Land
                { 1.0 }
                else
                { 0.0 };
        }

        guassian_blur(&mut height_map, 2);

        let elevation_size = height_map.size();
        let mut elevation: Vec<u8> = vec![];
        elevation.resize(elevation_size.width as usize * elevation_size.height as usize * size_of::<u32>(), 0);
        let elevation_slice = bytemuck::cast_slice_mut::<u8, u32>(&mut elevation);
        for i in 0..elevation_size.num_elements()
        {
            elevation_slice[i] = (height_map[i].clamp(0.0, 1.0) * 255.0) as u32;
        }
        let mut elevation_image = Image::new
        (
            Extent3d{
                width: elevation_size.width as u32,
                height: elevation_size.height as u32,
                depth_or_array_layers: 1 },
            TextureDimension::D2,
            elevation,
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::RENDER_WORLD
        );
        elevation_image.sampler = bevy::image::ImageSampler::Descriptor
        (
            bevy::image::ImageSamplerDescriptor {
                label: Some("elevation".to_owned()),
                mag_filter: bevy::image::ImageFilterMode::Nearest,
                min_filter: bevy::image::ImageFilterMode::Nearest,
                mipmap_filter: bevy::image::ImageFilterMode::Nearest,
                ..Default::default()
            }
        );

        images.add(elevation_image)
    };

    let custom_material = materials.add
    (
        TerrainMaterial
        {
            color: Color::WHITE.into(),
            elevation: elevation_handle,
            color_palette: asset_server.load("textures/palette.png"),
        }
    );

    let scale = vis_tuning.cell_size * Vec2::new(size.width as f32, size.height as f32);
    let v_pos = vec!
    [
        [0.0 * scale.x, 0.0 * scale.y, layers::TERRAIN], // TL
        [1.0 * scale.x, 0.0 * scale.y, layers::TERRAIN], // TR
        [0.0 * scale.x, 1.0 * scale.y, layers::TERRAIN], // BL
        [1.0 * scale.x, 1.0 * scale.y, layers::TERRAIN], // BR
    ];
    let v_color: Vec<[f32; 4]> = vec![LinearRgba::WHITE.to_f32_array(); 4];
    let v_uv: Vec<[f32; 2]> = vec!
    [
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];
    let v_normal: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

    let indices = vec!
    [
        0, 2, 1,
        1, 2, 3
    ];

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

pub struct TerrainVisPlugin;

impl Plugin for TerrainVisPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(Material2dPlugin::<TerrainMaterial>::default())
            ;
    }
}
