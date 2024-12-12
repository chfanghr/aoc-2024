use std::iter;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Grid<T>(pub Vec<Vec<T>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset(pub isize, pub isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridSize(pub usize, pub usize);

impl Position {
    #[inline]
    pub fn checked_add_offset(&self, offset: Offset, grid_size: GridSize) -> Option<Self> {
        let Position(row_index, col_index) = self;
        let Offset(row_offset, col_offset) = offset;
        let GridSize(rows, cols) = grid_size;
        let row_index = row_index
            .checked_add_signed(row_offset)
            .filter(|row_index| *row_index < rows)?;
        let col_index = col_index
            .checked_add_signed(col_offset)
            .filter(|col_index| *col_index < cols)?;
        Some(Position(row_index, col_index))
    }
}

impl<T> Grid<T> {
    pub fn new(inner: Vec<Vec<T>>) -> Self {
        Self(inner)
    }

    pub fn fill_with(elm: T, grid_size: GridSize) -> Self
    where
        T: Clone,
    {
        let GridSize(cols, rows) = grid_size;
        Grid(
            iter::repeat(iter::repeat(elm).take(cols).collect_vec())
                .take(rows)
                .collect_vec(),
        )
    }

    #[inline]
    pub fn size(&self) -> GridSize {
        let rows = self.0.len();
        let cols = self.0.get(0).map(|row| row.len()).unwrap_or(0);
        GridSize(rows, cols)
    }

    #[inline]
    pub fn must_get_cell<'a>(&'a self, position: Position) -> &'a T {
        let Position(row_index, col_index) = position;
        self.0.get(row_index).unwrap().get(col_index).unwrap()
    }

    #[inline]
    pub fn must_get_mut_cell<'a>(&'a mut self, position: Position) -> &'a mut T {
        let Position(row_index, col_index) = position;
        self.0
            .get_mut(row_index)
            .unwrap()
            .get_mut(col_index)
            .unwrap()
    }

    pub fn positions<'a>(&'a self) -> impl 'a + Iterator<Item = Position> {
        let GridSize(rows, cols) = self.size();
        (0..rows)
            .into_iter()
            .map(move |row_index| {
                (0..cols)
                    .into_iter()
                    .map(move |col_index| Position(row_index, col_index))
            })
            .flatten()
    }
}
