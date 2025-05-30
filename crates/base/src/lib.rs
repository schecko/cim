
pub mod array2;
pub mod assets;
pub mod debug;
pub mod extents;
pub mod ronx;
pub mod tuning;
pub mod const_default;

pub fn convert<Out, In, const SIZE: usize>( input: &[In; SIZE] ) -> [Out; SIZE] where
    Out: From<In> + Copy + Default,
    In: Into<Out> + Copy
{
    let mut out = [Out::default(); SIZE];
    let mut i = 0;
    while i < SIZE
    {
        out[i] = Out::from(input[i]);
        i += 1;
    }
    out
}

pub fn hello_base() 
{
    println!("Hello, base!");
}
