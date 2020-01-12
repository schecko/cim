
use cgmath::*;
use cgmath::prelude::*;
use crate::*;

pub struct Renderer;

impl Renderer {
    pub fn render(&mut self, game_state: &mut GameState, camera: &mut Camera) {
        let (grid_width, grid_height) = game_state.grid.dim();
        let signed_width = grid_width as isize;
        let signed_height = grid_height as isize;

        let mut rect_positions: Vec<_> = game_state.grid.indexed_iter().map(|((x, y), grid)| {
            let loc_z = match game_state.cursor == (x, y).into() {
                true => 1.0,
                false => 0.0,
            };
            let loc_x = x as f32 + 0.5;
            let loc_y = y as f32;
            // TODO: game grid lines rather than spacers.
            [
                Vector3::new(2.0 * loc_x, 2.0 * loc_y, loc_z),
                grid.biome.color(),
            ]
        }).collect();
        game_state.quad_instance_data.sub_data(&mut rect_positions);

        let mut unit_positions: Vec<_> = game_state.grid
            .indexed_iter()
            .filter(|(_, cell)| cell.unit.is_some())
            .map(|((x, y), cell)| {
                let loc_x = x as f32 + 0.5;
                let loc_y = y as f32;
                let loc_z = match game_state.cursor == (x, y).into() {
                    true => 2.0,
                    false => 1.0,
                };

                [
                    Vector3::new(loc_x * 2.0, loc_y * 2.0, loc_z),
                    Vector3::new(0.5, 0.5, 0.5),
                ]
            }).collect();

        unit_positions.append(&mut game_state.grid
            .indexed_iter()
            .filter(|(_, cell)| cell.structure.is_some())
            .map(|((x, y), cell)| {
                let loc_x = x as f32 + 0.5;
                let loc_y = y as f32;
                let loc_z = match game_state.cursor == (x, y).into() {
                    true => 2.0,
                    false => 1.0,
                };

                [
                    Vector3::new(loc_x * 2.0, loc_y * 2.0, loc_z),
                    Vector3::new(0.7, 0.7, 0.7),
                ]
            }).collect()
        );

        game_state.cube_instance_data.data(&mut unit_positions, gl::STATIC_DRAW);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
            gl::ClearColor(1.0, 0.5, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            game_state.solid.set_use();

            let decomp = Decomposed {
                scale: 1.0,
                rot: Basis3::look_at(Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 0.0)),
                disp: Vector3::new(0.0, 0.0, 0.0),
            };

            // TODO: proper screen ratio
            let proj: Matrix4<f32> = camera.projection;
            let mut rot_raw = Decomposed::<Vector3<f32>, Quaternion<f32>>::one();
            rot_raw.rot = camera.view.rot;
            let mut disp_raw = Decomposed::<Vector3<f32>, Quaternion<f32>>::one();
            disp_raw.disp = camera.view.disp;
            disp_raw.disp.x += game_state.cursor.loc.0 as f32 * -2.0;
            disp_raw.disp.y += game_state.cursor.loc.1 as f32 * -2.0;

            let rot: Matrix4<f32> = rot_raw.into();
            let disp: Matrix4<f32> = disp_raw.into();
            let view = rot * disp;

            let model: Matrix4<f32> = decomp.into();

            let model_loc = game_state.solid.get_uniform_location("model");
            let view_loc = game_state.solid.get_uniform_location("view");
            let proj_loc = game_state.solid.get_uniform_location("proj");

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj.as_ptr());

            assert!(gl::GetError() == 0);
            gl::BindVertexArray(game_state.quad_vao.0);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, crate::RECT.len() as i32, rect_positions.len() as i32);

            gl::BindVertexArray(game_state.cube_vao.0);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, crate::CUBE.len() as i32, unit_positions.len() as i32);
            assert!(gl::GetError() == 0);
        }
    }
}
