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
    IndicesOutOfBounds(isize, isize),
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
            Error::IndicesOutOfBounds(x, y) => write!(f, "Array2 -- indices ({x}, {y}) out of bounds"),
            Error::IndexOutOfBounds(index) => write!(f, "Array2 -- index {index} out of bounds"),
            Error::DimensionMismatch => write!(f, "Array2 -- dimension mismatch"),
            Error::NotEnoughValues => write!(f, "Array2 -- not enough values"),
        }
    }
}

impl std::error::Error for Error {}

#[allow(dead_code)]
impl<T> Array2<T>
{
    pub fn new(width: isize, height: isize) -> Self
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

    pub fn from_row_major(
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

    pub fn from_column_major(
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
        let array = size.positions_row_major()
            .map(|(x, y)| {
                let index = (y * size.height + x) as usize;
                values[index].clone()
            })
            .collect();
        Ok(Array2 {
            array,
            size: size,
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

    pub fn from_iter_row_major<I>(
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

    pub fn from_iter_column_major<I>(
        iterator: I,
        size: Extents
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
        T: Clone,
    {
        let array_column_major = iterator.take(size.num_elements()).collect::<Vec<_>>();
        Array2::from_column_major(&array_column_major, size)
            .map_err(|_| Error::NotEnoughValues)
    }

    pub fn height(&self) -> isize
    {
        self.size.height
    }

    pub fn width(&self) -> isize
    {
        self.size.width
    }

    pub fn size(&self) -> Extents
    {
        self.size
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T>
    {
        self.size.get_index_row_major(x, y).map(|index| &self.array[index])
    }

    pub fn get_index(&self, x: isize, y: isize) -> Option<usize>
    {
        self.size.get_index_row_major(x, y)
    }

    pub fn get_row_major(&self, index: usize) -> Option<&T>
    {
        self.array.get(index)
    }

    pub fn get_column_major(&self, index: usize) -> Option<&T>
    {
        let x = dbg!(index as isize % self.size.height);
        let y = dbg!(dbg!(index as isize) / self.size.height);
        self.get(x, y)
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T>
    {
        self.get_index(x, y)
            .map(move |index| &mut self.array[index])
    }

    pub fn get_mut_row_major(&mut self, index: usize) -> Option<&mut T>
    {
        self.array.get_mut(index)
    }

    pub fn get_mut_column_major(&mut self, index: usize) -> Option<&mut T>
    {
        let x = index as isize % self.size.height;
        let y = index as isize / self.size.height;
        self.get_mut(x, y)
    }

    pub fn set(&mut self, x: isize, y: isize, element: T) -> Result<(), Error>
    {
        self.get_mut(x, y)
            .map(|e|
             {
                *e = element;
            })
            .ok_or(Error::IndicesOutOfBounds(x, y))
    }

    pub fn set_row_major(&mut self, index: usize, element: T) -> Result<(), Error>
    {
        self.get_mut_row_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    pub fn set_column_major(&mut self, index: usize, element: T) -> Result<(), Error>
    {
        self.get_mut_column_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    pub fn elements_row_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone
    {
        self.array.iter()
    }

    pub fn elements_column_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone
    {
        self.indices_column_major().map(move |i| &self[i])
    }

    pub fn row_iter(&self, y: isize) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error>
    {
        let start = self
            .get_index(0, y)
            .ok_or(Error::IndicesOutOfBounds(0, y))?;
        let end = start + (self.size.width as usize);
        Ok(self.array[start..end].iter())
    }

    pub fn column_iter(
        &self,
        x: isize,
    ) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error>
    {
        if x >= self.size.width
        {
            return Err(Error::IndicesOutOfBounds(x, 0));
        }
        Ok((0..self.size.height).map(move |y| &self[(x, y)]))
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

    pub fn columns_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone
    {
        (0..self.size.width).map(move |x|
        {
            self.column_iter(x)
                .expect("Array2 -- columns_iter should never fail")
        })
    }

    pub fn as_row_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.elements_row_major_iter().cloned().collect()
    }

    pub fn as_column_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.elements_column_major_iter().cloned().collect()
    }

    pub fn positions_row_major(&self) -> impl DoubleEndedIterator<Item = Point> + Clone
    {
        self.size.positions_row_major()
    }

    pub fn indices_column_major(&self) -> impl DoubleEndedIterator<Item = Point> + Clone
    {
        self.size.positions_column_major()
    }

    pub fn enumerate_row_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Point, &T)> + Clone
    {
        self.positions_row_major().map(move |i| (i, &self[i]))
    }

    pub fn enumerate_column_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Point, &T)> + Clone
    {
        self.indices_column_major().map(move |i| (i, &self[i]))
    }
}

impl<T> Index<Point> for Array2<T>
{
    type Output = T;

    fn index(&self, (x, y): Point) -> &Self::Output
    {
        self.get(x, y)
            .unwrap_or_else(|| panic!("Array2 -- Index indices {}, {} out of bounds", x, y))
    }
}

impl<T> IndexMut<Point> for Array2<T>
{
    fn index_mut(&mut self, (x, y): Point) -> &mut Self::Output
    {
        self.get_mut(x, y)
            .unwrap_or_else(|| panic!("Array2 -- Index mut indices {}, {} out of bounds", x, y))
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

fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T>
{
    nested.iter().flat_map(|row| row.clone()).collect()
}

