
use cgmath::*;
use specs::prelude::*;
use cgmath::prelude::*;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, crate::GridPosition>, ReadExpect<'a, crate::GameState>, ReadExpect<'a, crate::Camera>);

    fn run(&mut self, (grid_pos, game_state, camera): Self::SystemData) {
        use specs::Join;

        let (grid_width, grid_height) = game_state.grid.dim();
        let signed_width = grid_width as isize;
        let signed_height = grid_height as isize;

        let mut rect_positions: Vec<_> = game_state.grid.indexed_iter().map(|((x, y), grid)| {
            let loc_z = match game_state.cursor == (x, y).into() {
                true => 1.0,
                false => 0.0,
            };
            let loc_x = (-signed_width / 2 + x as isize) as f32 + 0.5;
            let loc_y = (-signed_height / 2 + y as isize) as f32;
            // TODO: game grid lines rather than spacers.
            [
                Vector3::new(2.0 * loc_x, 2.0 * loc_y, loc_z),
                grid.biome.color(),
            ]
        }).collect();
        game_state.quad_instance_data.sub_data(&mut rect_positions);

        let mut unit_positions = grid_pos
            .join()
            .map(|pos| {
                let loc_x = (-signed_width / 2 + pos.xy.0 as isize) as f32 + 0.5;
                let loc_y = (-signed_height / 2 + pos.xy.1 as isize) as f32;
                let loc_z = match game_state.cursor == pos.xy.into() {
                    true => 2.0,
                    false => 1.0,
                };

                [
                    Vector3::new(loc_x * 2.0, loc_y * 2.0, loc_z),
                    Vector3::new(0.5, 0.5, 0.5),
                ]
            })
            .collect();

        game_state.cube_instance_data.data(&mut unit_positions, gl::STATIC_DRAW);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(1.0, 0.5, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            game_state.solid.set_use();

            let decomp = Decomposed {
                scale: 1.0,
                rot: Quaternion::new(0.0, 0.0, 0.0, 0.0),
                disp: Vector3::new(0.0, 0.0, 0.0),
            };

            // TODO: proper screen ratio
            let proj: Matrix4<f32> = camera.projection;
            let view: Matrix4<f32> = camera.view.into();

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
