use crate::extents::Extents;

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
    IndicesOutOfBounds(usize, usize),
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
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Clone + Default,
    {
        let total_len = width * height;
        let array = vec![T::default(); total_len];
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

    pub fn from_rows(values: &[Vec<T>]) -> Result<Self, Error>
    where
        T: Clone,
    {
        let width = values.get(0).map(Vec::len).unwrap_or(0);
        if values.iter().any(|row| row.len() != width)
        {
            return Err(Error::DimensionMismatch);
        }
        Ok(Array2 {
            array: flatten(values),
            size: (width, values.len()).into(),
        })
    }

    pub fn from_columns(values: &[Vec<T>]) -> Result<Self, Error>
    where
        T: Clone,
    {
        let column_len = values.get(0).map(Vec::len).unwrap_or(0);
        if !values.iter().all(|column| column.len() == column_len)
        {
            return Err(Error::DimensionMismatch);
        }
        let size = Extents::new( values.len(), column_len );
        let array = size.positions_row_major()
            .map(|(x, y)| values[y][x].clone())
            .collect();
        Ok(Array2 {
            array,
            size,
        })
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
        width: usize,
        height: usize,
    ) -> Result<Self, Error>
    where
        T: Clone,
    {
        let total_len = width * height;
        if total_len != values.len()
        {
            return Err(Error::DimensionMismatch);
        }
        let size = Extents{ width, height };
        let array = size.positions_row_major()
            .map(|(x, y)| {
                let index = y * height + x;
                values[index].clone()
            })
            .collect();
        Ok(Array2 {
            array,
            size: size,
        })
    }

    pub fn filled_with(element: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        let total_len = width * height;
        let array = vec![element; total_len];
        Array2 {
            array,
            size: (width, height).into(),
        }
    }

    pub fn filled_by<F>(mut generator: F, width: usize, height: usize) -> Self
    where
        F: FnMut() -> T,
    {
        let total_len = width * height;
        let array = (0..total_len).map(|_| generator()).collect();
        Array2 {
            array,
            size: (width, height).into(),
        }
    }

    pub fn from_iter_row_major<I>(
        iterator: I,
        width: usize,
        height: usize,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
    {
        let total_len = width * height;
        let array = iterator.take(total_len).collect::<Vec<_>>();
        if array.len() != total_len
        {
            return Err(Error::NotEnoughValues);
        }
        Ok(Array2 {
            array,
            size: (width, height).into(),
        })
    }

    pub fn from_iter_column_major<I>(
        iterator: I,
        width: usize,
        height: usize,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
        T: Clone,
    {
        let total_len = width * height;
        let array_column_major = iterator.take(total_len).collect::<Vec<_>>();
        Array2::from_column_major(&array_column_major, width, height)
            .map_err(|_| Error::NotEnoughValues)
    }

    pub fn height(&self) -> usize
    {
        self.size.height
    }

    pub fn width(&self) -> usize
    {
        self.size.width
    }

    pub fn size(&self) -> Extents
    {
        self.size
    }

    pub fn num_values(&self) -> usize
    {
        self.size.height * self.size.width
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T>
    {
        self.size.get_index_row_major(x, y).map(|index| &self.array[index])
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize>
    {
        self.size.get_index_row_major(x, y)
    }

    pub fn get_row_major(&self, index: usize) -> Option<&T>
    {
        self.array.get(index)
    }

    pub fn get_column_major(&self, index: usize) -> Option<&T>
    {
        let x = dbg!(index % self.size.height);
        let y = dbg!(dbg!(index) / self.size.height);
        self.get(x, y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T>
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
        let x = index % self.size.height;
        let y = index / self.size.height;
        self.get_mut(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, element: T) -> Result<(), Error>
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

    pub fn row_iter(&self, y: usize) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error>
    {
        let start = self
            .get_index(0, y)
            .ok_or(Error::IndicesOutOfBounds(0, y))?;
        let end = start + self.size.width;
        Ok(self.array[start..end].iter())
    }

    pub fn column_iter(
        &self,
        x: usize,
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

    pub fn positions_row_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone
    {
        self.size.positions_row_major()
    }

    pub fn indices_column_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone
    {
        self.size.positions_column_major()
    }

    pub fn enumerate_row_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone
    {
        self.positions_row_major().map(move |i| (i, &self[i]))
    }

    pub fn enumerate_column_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone
    {
        self.indices_column_major().map(move |i| (i, &self[i]))
    }
}

impl<T> Index<(usize, usize)> for Array2<T>
{
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output
    {
        self.get(x, y)
            .unwrap_or_else(|| panic!("Array2 -- Index indices {}, {} out of bounds", x, y))
    }
}

impl<T> IndexMut<(usize, usize)> for Array2<T>
{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output
    {
        self.get_mut(x, y)
            .unwrap_or_else(|| panic!("Array2 -- Index mut indices {}, {} out of bounds", x, y))
    }
}

fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T>
{
    nested.iter().flat_map(|row| row.clone()).collect()
}

