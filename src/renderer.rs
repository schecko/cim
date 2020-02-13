
use cgmath::*;
use crate::*;
use pipeline::*;

static VERTEX: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 aVertOffset;
    layout (location = 1) in vec3 aWorldPos;
    layout (location = 2) in vec3 aColor;

    out vec3 vColor;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;

    void main() {
        gl_Position = proj * view * model * vec4(aWorldPos + aVertOffset, 1.0);
        vColor = aColor;
    }
"#;

static FRAGMENT: &str = r#"
    #version 330 core

    out vec4 FragColor;
    in vec3 vColor;

    void main() {
        FragColor = vec4(vColor, 1.0);
    }
"#;

pub static RECT: [[f32; 3]; 6] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],

    [-1.0, -1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, 1.0, 0.0],
];

pub static CUBE: [[f32; 3]; 36] = [
    [-1.0, -1.0, -1.0],
    [-1.0, -1.0,  1.0],
    [-1.0,  1.0,  1.0],
    [1.0,  1.0, -1.0 ],
    [-1.0, -1.0, -1.0],
    [-1.0,  1.0, -1.0],
    [1.0, -1.0,  1.0 ],
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0 ],
    [1.0,  1.0, -1.0 ],
    [1.0, -1.0, -1.0 ],
    [-1.0, -1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [-1.0,  1.0,  1.0],
    [-1.0,  1.0, -1.0],
    [1.0, -1.0,  1.0 ],
    [-1.0, -1.0,  1.0],
    [-1.0, -1.0, -1.0],
    [-1.0,  1.0,  1.0],
    [-1.0, -1.0,  1.0],
    [1.0, -1.0,  1.0 ],
    [1.0,  1.0,  1.0 ],
    [1.0, -1.0, -1.0 ],
    [1.0,  1.0, -1.0 ],
    [1.0, -1.0, -1.0 ],
    [1.0,  1.0,  1.0 ],
    [1.0, -1.0,  1.0 ],
    [1.0,  1.0,  1.0 ],
    [1.0,  1.0, -1.0 ],
    [-1.0,  1.0, -1.0],
    [1.0,  1.0,  1.0 ],
    [-1.0,  1.0, -1.0],
    [-1.0,  1.0,  1.0],
    [1.0,  1.0,  1.0 ],
    [-1.0,  1.0,  1.0],
    [1.0, -1.0,  1.0 ],
];

pub struct Renderer {
    solid: Pipeline,

    quad_data: Buffer,
    quad_instance_data: Buffer,
    quad_vao: Vao,

    cube_data: Buffer,
    cube_instance_data: Buffer,
    cube_vao: Vao,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.cube_data.0 as *const _);
            gl::DeleteBuffers(1, &self.cube_instance_data.0 as *const _);
            gl::DeleteBuffers(1, &self.quad_data.0 as *const _);
            gl::DeleteBuffers(1, &self.quad_instance_data.0 as *const _);

            gl::DeleteVertexArrays(1, &self.quad_vao.0 as *const _);
            gl::DeleteVertexArrays(1, &self.cube_vao.0 as *const _);
        }
    }
}

impl Renderer {
    pub fn new(game_state: &GameState) -> Result<Self, String> {
        let quad_data = Buffer::new();
        let quad_instance_data = Buffer::new();
        let quad_vao = Vao::new(quad_data, quad_instance_data);
        quad_data.data(&mut RECT.to_vec(), gl::STATIC_DRAW);

        let mut rect_positions: Vec<_> = game_state.grid.indexed_iter().map(|((_x, _y), _grid)| {
            [
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 0.0),
            ]
        }).collect();
        quad_instance_data.data(&mut rect_positions, gl::DYNAMIC_DRAW);

        let cube_data = Buffer::new();
        cube_data.data(&mut CUBE.to_vec(), gl::STATIC_DRAW);
        let cube_instance_data = Buffer::new();
        let cube_vao = Vao::new(cube_data, cube_instance_data);

        let renderer = Renderer {
            solid: Pipeline::new(VERTEX, FRAGMENT)?,

            quad_data,
            quad_instance_data,
            quad_vao,

            cube_data,
            cube_instance_data,
            cube_vao,
        };

        Ok(renderer)
    }

    pub fn render(&mut self, game_state: &mut GameState, camera: &mut Camera) {
        let (grid_width, grid_height) = game_state.grid.dim();
        let signed_width = grid_width as isize;
        let signed_height = grid_height as isize;

        // TODO: proper screen ratio
        let proj: Matrix4<f32> = camera.projection;
        let mut rot_raw = Decomposed::<Vector3<f32>, Quaternion<f32>>::one();
        rot_raw.rot = camera.view.rot;
        let mut disp_raw = Decomposed::<Vector3<f32>, Quaternion<f32>>::one();
        disp_raw.disp = camera.view.disp;
        disp_raw.disp.x += game_state.cursor.loc.x as f32 * -2.0;
        disp_raw.disp.y += game_state.cursor.loc.y as f32 * -2.0;

        let rot: Matrix4<f32> = rot_raw.into();
        let disp: Matrix4<f32> = disp_raw.into();
        let view = rot * disp;

        let decomp = Decomposed {
            scale: 1.0,
            rot: Basis3::look_at(Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 0.0)),
            disp: Vector3::new(0.0, 0.0, 0.0),
        };
        let model: Matrix4<f32> = decomp.into();

        let blah = |coords: Vector2<f32>| -> Vector3<f32> {
            let inv_proj = proj.inverse_transform().unwrap();
            let inv_view = view.transpose();
            let world_coord_p1 = inv_view * inv_proj * Vector4::new(coords.x, coords.y, 0., 1.);
            let world_coord_p2 = inv_view * inv_proj * Vector4::new(coords.x, coords.y, 1., 1.);
            let camera_dir = (world_coord_p1.truncate() / world_coord_p1.w) - (world_coord_p2.truncate() / world_coord_p2.w);
            let ray_dir = camera_dir.normalize();

            let plane_point = Vector3::new(0., 0., 0.);
            let plane_normal = Vector3::new(0., 0., 1.);
            let d = dot(plane_point - disp_raw.disp, plane_normal) / dot(ray_dir, plane_normal);
            let intersection = -disp_raw.disp - ray_dir * d;
            intersection
        };
        let top_right = blah(Vector2::new(1., 1.)) / 2.;
        // NOTE: use width from top corners due to perspective view
        let top_left = blah(Vector2::new(-1., 1.)) / 2.;
        let bottom_left = blah(Vector2::new(-1., -1.)) / 2.;

        let mut top_right_index = top_right.truncate().cast::<isize>().unwrap() + Vector2::new(2, 1); // add one to counter floor from cast
        let mut top_left_index = top_left.truncate().cast::<isize>().unwrap() + Vector2::new(0, 1); // add one to counter floor from cast
        let mut bottom_left_index = bottom_left.truncate().cast::<isize>().unwrap();
        top_right_index.x = num::clamp(top_right_index.x, 0, signed_width);
        top_right_index.y = num::clamp(top_right_index.y, 0, signed_height);
        top_left_index.x = num::clamp(top_left_index.x, 0, signed_width);
        top_left_index.y = num::clamp(top_left_index.y, 0, signed_height);
        bottom_left_index.x = num::clamp(bottom_left_index.x, 0, signed_width);
        bottom_left_index.y = num::clamp(bottom_left_index.y, 0, signed_height);

        // TODO: perspective projection onto the grid shouldnt be a perfect square. slight speed
        // improvment?
        let viewable_grid = game_state.grid.slice(s![top_left_index.x..top_right_index.x, bottom_left_index.y..top_right_index.y]);

        let mut rect_positions: Vec<_> = viewable_grid
            .indexed_iter()
            .map(|((x_i, y_i), grid)| {
                let x = x_i + top_left_index.x as usize;
                let y = y_i + bottom_left_index.y as usize;
                let loc_z = match game_state.cursor == (x, y).into() {
                    true => 1.0,
                    false => 0.0,
                };
                let loc_x = 2.0 * x as f32 + 0.5;
                let loc_y = 2.0 * y as f32;
                // TODO: game grid lines rather than spacers.
                [
                    Vector3::new(loc_x, loc_y, loc_z),
                    grid.biome.color()
                ]
            }).collect();
        self.quad_instance_data.sub_data(&mut rect_positions);

        let mut unit_positions: Vec<_> = viewable_grid
            .indexed_iter()
            .filter(|(_, cell)| cell.unit.is_some())
            .map(|((x_i, y_i), _cell)| {
                let x = x_i + top_left_index.x as usize;
                let y = y_i + bottom_left_index.y as usize;
                let loc_x = 2. * x as f32 + 0.5;
                let loc_y = 2. * y as f32;
                let loc_z = match game_state.cursor == (x, y).into() {
                    true => 2.,
                    false => 1.,
                };

                [
                    Vector3::new(loc_x, loc_y, loc_z),
                    Vector3::new(0.5, 0.5, 0.5),
                ]
            }).collect();

        unit_positions.append(&mut viewable_grid
            .indexed_iter()
            .filter(|(_, cell)| cell.structure.is_some())
            .map(|((x_i, y_i), _cell)| {
                let x = x_i + top_left_index.x as usize;
                let y = y_i + bottom_left_index.y as usize;
                let loc_x = 2. * x as f32 + 0.5;
                let loc_y = 2. * y as f32;
                let loc_z = match game_state.cursor == (x, y).into() {
                    true => 2.0,
                    false => 1.0,
                };

                [
                    Vector3::new(loc_x, loc_y, loc_z),
                    Vector3::new(0.7, 0.0, 0.7),
                ]
            }).collect()
        );

        self.cube_instance_data.data(&mut unit_positions, gl::STATIC_DRAW);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.solid.set_use();

            let model_loc = self.solid.get_uniform_location("model");
            let view_loc = self.solid.get_uniform_location("view");
            let proj_loc = self.solid.get_uniform_location("proj");

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj.as_ptr());

            assert!(gl::GetError() == 0);
            gl::BindVertexArray(self.quad_vao.0);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, crate::RECT.len() as i32, rect_positions.len() as i32);

            gl::BindVertexArray(self.cube_vao.0);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, crate::CUBE.len() as i32, unit_positions.len() as i32);
            assert!(gl::GetError() == 0);
        }
    }
}
