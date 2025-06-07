

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Point(glam::IVec2);

impl std::ops::Deref for Point
{
    type Target = glam::IVec2;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl std::ops::DerefMut for Point
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

impl From<glam::IVec2> for Point
{
    fn from(input: glam::IVec2) -> Self
    {
        Self(input)
    }
}

impl From<[i32; 2]> for Point
{
    #[inline]
    fn from(a: [i32; 2]) -> Self
    {
        Self(glam::IVec2::new(a[0], a[1]))
    }

}

impl From<Point> for [i32; 2]
{
    #[inline]
    fn from(v: Point) -> Self
    {
        [v.0.x, v.0.y]
    }
}

impl From<(i32, i32)> for Point
{
    #[inline]
    fn from(t: (i32, i32)) -> Self
    {
        Self(glam::IVec2::new(t.0, t.1))
    }
}

impl From<Point> for (i32, i32)
{
    #[inline]
    fn from(v: Point) -> Self
    {
        (v.0.x, v.0.y)
    }
}

impl Point
{
    pub fn new(x: i32, y: i32) -> Self
    {
        Self(glam::IVec2::new(x, y))
    }

    pub fn is_adjacent(&self, other: &Point) -> bool
    {
        (self.x - other.x).abs() == 1 || (self.y - other.y).abs() == 1 
    }

    // 2 2 2 2 2
    // 2 1 1 1 2
    // 2 1 0 1 2 // 0 is the poit of reference
    // 2 1 1 1 2
    // 2 2 2 2 2
    pub fn onion_distance(&self, other: &Point) -> u32
    {
        std::cmp::max((self.x - other.x).abs(), (self.y - other.y).abs()) as u32
    }

    pub fn manhattan_distance(&self, other: &Point) -> u32
    {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

