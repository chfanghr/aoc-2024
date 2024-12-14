use std::{
    iter,
    ops::{Range, RangeBounds},
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Grid<T>(pub Vec<Vec<T>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub row_index: usize,
    pub col_index: usize,
}

impl Position {
    pub fn new(row_index: usize, col_index: usize) -> Position {
        Position {
            row_index,
            col_index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset {
    pub row_offset: isize,
    pub col_offset: isize,
}

impl Offset {
    pub fn new(row_offset: isize, col_offset: isize) -> Offset {
        Offset {
            row_offset,
            col_offset,
        }
    }

    pub const DOWN: Offset = Offset {
        row_offset: 1,
        col_offset: 0,
    };

    pub const UP: Offset = Offset {
        row_offset: -1,
        col_offset: 0,
    };

    pub const LEFT: Offset = Offset {
        row_offset: 0,
        col_offset: -1,
    };

    pub const RIGHT: Offset = Offset {
        row_offset: 0,
        col_offset: 1,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constraints<R>
where
    R: RangeBounds<usize>,
{
    pub row_range: R,
    pub col_range: R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridSize(pub usize, pub usize);

impl Into<Constraints<Range<usize>>> for GridSize {
    fn into(self) -> Constraints<Range<usize>> {
        Constraints {
            row_range: 0..self.0,
            col_range: 0..self.1,
        }
    }
}

impl Position {
    #[inline]
    pub fn checked_add_offset<R>(&self, offset: Offset, constraints: Constraints<R>) -> Option<Self>
    where
        R: RangeBounds<usize>,
    {
        let row_index = self
            .row_index
            .checked_add_signed(offset.row_offset)
            .filter(|row_index| constraints.row_range.contains(row_index))?;
        let col_index = self
            .col_index
            .checked_add_signed(offset.col_offset)
            .filter(|col_index| constraints.col_range.contains(col_index))?;
        Some(Position::new(row_index, col_index))
    }
}

impl Offset {
    #[inline]
    pub fn unchecked_add(&self, r: Offset) -> Offset {
        Offset::new(
            self.row_offset + r.row_offset,
            self.col_offset + r.col_offset,
        )
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
        self.0
            .get(position.row_index)
            .unwrap()
            .get(position.col_index)
            .unwrap()
    }

    #[inline]
    pub fn must_get_mut_cell<'a>(&'a mut self, position: Position) -> &'a mut T {
        self.0
            .get_mut(position.row_index)
            .unwrap()
            .get_mut(position.col_index)
            .unwrap()
    }

    pub fn positions<'a>(&'a self) -> impl 'a + Iterator<Item = Position> {
        let GridSize(rows, cols) = self.size();
        (0..rows)
            .into_iter()
            .map(move |row_index| {
                (0..cols)
                    .into_iter()
                    .map(move |col_index| Position::new(row_index, col_index))
            })
            .flatten()
    }
}
