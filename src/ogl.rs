
use std::mem;

#[derive(Clone, Copy)]
pub struct Vao(pub u32);

#[derive(Clone, Copy)]
pub struct Buffer(pub u32);

impl Buffer {
    pub fn new() -> Self {
        let vbo = unsafe {
            let mut vbo: u32 = 0;
            gl::GenBuffers(1, &mut vbo as *mut _);
            vbo
        };
        Buffer(vbo)
    }

    pub fn data<T>(&self, data: &mut Vec<T>, store_type: u32) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.0);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<T>()) as isize,
                data.as_mut_ptr() as *mut _,
                store_type
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn sub_data<T>(&self, data: &mut Vec<T>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.0);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (data.len() * mem::size_of::<T>()) as isize,
                data.as_mut_ptr() as *mut _
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Vao {
    pub fn new(model_data: Buffer, instance_data: Buffer) -> Self {
        let vao = unsafe {
            let mut vao: u32 = 0;
            gl::GenVertexArrays(1, &mut vao as *mut _);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, model_data.0);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as i32, std::ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, instance_data.0);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, std::ptr::null());
            gl::VertexAttribDivisor(1, 1);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>()) as *const _);
            gl::VertexAttribDivisor(2, 1);
            assert!(gl::GetError() == 0);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            vao
        };

        Vao(vao)
    }
}
