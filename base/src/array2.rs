use crate::extents::Extents;
use crate::extents::Point;

use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Array2<T>
{
    array: Vec<T>,
    size: Extents,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error
{
    IndicesOutOfBounds(Point),
    IndexOutOfBounds(usize),
    DimensionMismatch,
    NotEnoughValues,
}

impl Display for Error
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Error::IndicesOutOfBounds(pos) => write!(f, "Array2 -- indices ({}, {}) out of bounds", pos.x, pos.y),
            Error::IndexOutOfBounds(index) => write!(f, "Array2 -- index {index} out of bounds"),
            Error::DimensionMismatch => write!(f, "Array2 -- dimension mismatch"),
            Error::NotEnoughValues => write!(f, "Array2 -- not enough values"),
        }
    }
}

impl std::error::Error for Error {}

#[allow(dead_code)]
// 2 dimensional array, row major, indexed by either index or coordinates(index2)
impl<T> Array2<T>
{
    pub fn new(width: i32, height: i32) -> Self
    where
        T: Clone + Default,
    {
        let total_len = width * height;
        let array = vec![T::default(); total_len as usize];
        Array2 {
            array,
            size: (width, height).into(),
        }
    }

    pub fn from_size(size: Extents) -> Self
    where
        T: Clone + Default,
    {
        let array = vec![T::default(); size.num_elements()];
        Array2 {
            array,
            size,
        }
    }

    pub fn from_values(
        values: &[T],
        size: Extents,
    ) -> Result<Self, Error>
    where
        T: Clone,
    {
        if size.num_elements() != values.len()
        {
            return Err(Error::DimensionMismatch);
        }
        Ok(Array2 {
            array: values.to_vec(),
            size,
        })
    }

    pub fn filled_with(element: T, size: Extents) -> Self
    where
        T: Clone,
    {
        let array = vec![element; size.num_elements()];
        Array2 {
            array,
            size
        }
    }

    pub fn filled_by<F>(mut generator: F, size: Extents) -> Self
    where
        F: FnMut() -> T,
    {
        let array = (0..size.num_elements()).map(|_| generator()).collect();
        Array2 {
            array,
            size
        }
    }

    pub fn fill_with(&mut self, element: T)
    where
        T: Copy,
    {
        for cell in &mut self.array
        {
            *cell = element;
        }
    }

    pub fn fill_by<F>(&mut self, mut generator: F)
    where
        F: FnMut() -> T,
    {
        for cell in &mut self.array
        {
            *cell = generator();
        }
    }

    pub fn from_iter<I>(
        iterator: I,
        size: Extents,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
    {
        let array = iterator.take(size.num_elements()).collect::<Vec<_>>();
        if array.len() != size.num_elements()
        {
            return Err(Error::NotEnoughValues);
        }
        Ok(Array2 {
            array,
            size
        })
    }

    pub fn height(&self) -> i32
    {
        self.size.height
    }

    pub fn width(&self) -> i32
    {
        self.size.width
    }

    pub fn size(&self) -> Extents
    {
        self.size
    }

    pub fn get_by_index2(&self, pos: Point) -> Option<&T>
    {
        self.size.get_index(pos).map(|index| &self.array[index])
    }

    pub fn point_to_index(&self, pos: Point) -> Option<usize>
    {
        self.size.get_index(pos)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T>
    {
        self.array.get(index)
    }

    pub fn get_by_index2_mut(&mut self, pos: Point) -> Option<&mut T>
    {
        self.size.get_index(pos)
            .map(move |index| &mut self.array[index])
    }

    pub fn get_by_index_mut(&mut self, index: usize) -> Option<&mut T>
    {
        self.array.get_mut(index)
    }

    pub fn set_by_index2(&mut self, pos: Point, element: T) -> Result<(), Error>
    {
        self.get_by_index2_mut(pos)
            .map(|e|
             {
                *e = element;
            })
            .ok_or(Error::IndicesOutOfBounds(pos))
    }

    pub fn set_by_index(&mut self, index: usize, element: T) -> Result<(), Error>
    {
        self.get_by_index_mut(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    pub fn raw_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone
    {
        self.array.iter()
    }

    pub fn row_iter(&self, y: i32) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error>
    {
        let start = self
            .point_to_index((0, y).into())
            .ok_or(Error::IndicesOutOfBounds((0, y).into()))?;
        let end = start + (self.size.width as usize);
        Ok(self.array[start..end].iter())
    }

    pub fn rows_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone
    {
        (0..self.height()).map(move |y|
        {
            self.row_iter(y)
                .expect("Array2 -- rows_iter should never fail")
        })
    }

    pub fn index2_space(&self) -> impl DoubleEndedIterator<Item = Point> + Clone
    {
        self.size.index2_space()
    }

    pub fn enumerate(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Point, &T)> + Clone
    {
        self.index2_space().map(move |i| (i, &self[i]))
    }
}

impl<T> Index<Point> for Array2<T>
{
    type Output = T;

    fn index(&self, pos: Point) -> &Self::Output
    {
        self.get_by_index2(pos)
            .unwrap_or_else(|| panic!("Array2 -- Index indices {}, {} out of bounds", pos.x, pos.y))
    }
}

impl<T> IndexMut<Point> for Array2<T>
{
    fn index_mut(&mut self, pos: Point) -> &mut Self::Output
    {
        self.get_by_index2_mut(pos)
            .unwrap_or_else(|| panic!("Array2 -- Index mut indices {}, {} out of bounds", pos.x, pos.y))
    }
}

impl<T> Index<usize> for Array2<T>
{
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output
    {
        &self.array[i]
    }
}

impl<T> IndexMut<usize> for Array2<T>
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output
    {
        &mut self.array[i]
    }
}
