// how to implement a set?
// a set is a non organised collection of items on which certain operations can take place
// use super::column::ColumnIterator;
use std::collections::HashSet;
use super::digit::{DigitValue,Digit};
use super::griddimensions::{GridDimensions,Cell};
// use super::row::RowIterator;
// use super::section::{Section, SectionIterator};

pub struct Sudoku {
    grid: Vec<Cell>,
    grid_dimensions: GridDimensions,
}

pub struct SubSetIterator<'a> {
    data: Vec<&'a Cell>,
    current: usize,
}

impl<'a> SubSetIterator<'a> {
    pub fn new(data: Vec<&'a Cell>) -> Self {
        Self {
            data: data,
            current: 0,
        }
    }
}

impl<'a> Iterator for SubSetIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.data.len() {
            return None;
        }
        self.current += 1;
        Some(&self.data[self.current - 1])
    }
}

impl Sudoku {
    pub fn new(columns: usize, rows: usize, section_width: usize, section_height: usize) -> Self {
        let new_grid_dimensions = GridDimensions::new(columns, rows, section_width, section_height);
        Sudoku {
            grid: new_grid_dimensions.new_grid(),
            grid_dimensions: new_grid_dimensions,
        }
    }

    fn subset<'a>(&'a self, indices: Vec<usize>) -> impl Iterator<Item = &'a Cell> + 'a {
        let subset = indices.iter().map(|x| &self.grid[*x]).collect();
        SubSetIterator::new(subset)
    }

    pub fn row<'a>(&'a self, row: usize) -> impl Iterator<Item = &'a Cell> + 'a {
        let indices = self.grid_dimensions.get_indices_for_row(row);
        self.subset(indices)
    }

    pub fn column<'a>(&'a self, column: usize) -> impl Iterator<Item = &'a Cell> + 'a {
        let indices = self.grid_dimensions.get_indices_for_column(column);
        self.subset(indices)
    }

    pub fn section<'a>(&'a self, section: usize) -> impl Iterator<Item = &'a Cell> + 'a {
        let indices = self.grid_dimensions.get_indices_for_section(section);
        self.subset(indices)
    }

    fn unused_digits_for_iter<'a, I>(&self, vals: I) -> Option<HashSet<Digit>> 
    where
        I: Iterator<Item = &'a Cell>,
    {
        use super::digit::Digit::*;

        let mut result: HashSet<_> = [One,Two,Three, Four, Five,Six,Seven,Eight,Nine].iter().cloned().collect();
        let mut found_more_than_one_digit_in_set: bool = false;
        vals.filter(|x| x.get_value().is_some())
            .collect::<Vec<&Cell>>()
            .iter()
            .for_each(|x| found_more_than_one_digit_in_set |= !result.remove(&x.get_value().as_ref().unwrap()));
        if !found_more_than_one_digit_in_set {
            Some(result)
        }
        else {
            None
        }
    }    

    pub fn unused_digits_in_row(&self, row: usize) -> Option<HashSet<Digit>> {
        self.unused_digits_for_iter(self.row(row))
    }

    pub fn unused_digits_in_column(&self, column: usize) -> Option<HashSet<Digit>> {
        self.unused_digits_for_iter(self.column(column))
    }

    pub fn unused_digits_in_section(&self, section: usize) -> Option<HashSet<Digit>> {
        self.unused_digits_for_iter(self.section(section))
    }

    pub fn is_valid(&self) -> bool {
        self.grid_dimensions.is_valid()
    }

    pub fn solve(&self) -> bool {
        let mut row_sets: Vec<Option<HashSet<Digit>>> = Vec::new();
        let mut column_sets: Vec<Option<HashSet<Digit>>> = Vec::new();
        let mut section_sets: Vec<Option<HashSet<Digit>>> = Vec::new();

        (0..*self.grid_dimensions.get_column_count())
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|x| row_sets.push(self.unused_digits_in_row(*x)));
        (0..*self.grid_dimensions.get_row_count())
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|x| column_sets.push(self.unused_digits_in_column(*x)));
        (0..self.grid_dimensions.get_section_count())
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|x| section_sets.push(self.unused_digits_in_column(*x)));
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_make_an_empty_sudoku() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(mysudoku.is_valid(), true);
    }
    #[test]
    fn test_get_a_row() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        let count = mysudoku.row(2).count();
        assert_eq!(9, count);
    }
    #[test]
    fn test_get_a_column() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        let count = mysudoku.column(2).count();
        assert_eq!(9, count);
    }
    #[test]
    fn test_get_a_section() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        let count = mysudoku.section(2).count();
        assert_eq!(9, count);
    }
    #[test]
    fn test_get_a_non_existing_row() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        let count = mysudoku.row(10).count();
        assert_eq!(0, count);
    }
    #[test]
    fn test_get_a_non_existing_column() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        let count = mysudoku.column(10).count();
        assert_eq!(0, count);
    }
    #[test]
    fn test_get_a_non_existing_section() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        let count = mysudoku.section(10).count();
        assert_eq!(0, count);
    }
    #[test]
    fn test_unused_digits_in_row() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(mysudoku.unused_digits_in_row(3).unwrap().iter().count(), 9);
    }
    #[test]
    fn test_solve_sudoku() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(mysudoku.solve(), true);
    }
}
