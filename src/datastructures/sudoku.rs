// how to implement a set?
// a set is a non organised collection of items on which certain operations can take place
// use super::column::ColumnIterator;
use super::digit::{Digit, DigitValue};
use super::griddimensions::{Cell, GridDimensions};
use std::collections::HashSet;
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
        let subset = indices.iter().map(|x| self.get_cell(*x).unwrap()).collect();
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

    pub fn available_digits_for_cell(&self, cell: &Cell) -> Option<HashSet<Digit>> {
        let columnset = self.unused_digits_in_column(*cell.get_column());
        let rowset = self.unused_digits_in_row(*cell.get_row());
        let sectionset = self.unused_digits_in_section(*cell.get_section());

        if columnset.is_none() || rowset.is_none() || sectionset.is_none() {
            return None;
        }

        let mut intersection: HashSet<Digit> = columnset
            .unwrap()
            .intersection(&rowset.unwrap())
            .map(|x| *x)
            .collect();
        intersection = intersection
            .intersection(&sectionset.unwrap())
            .map(|x| *x)
            .collect();
        Some(intersection)
    }

    // pub fn sort_cells_by_freedom<'a>(&'a self) -> impl Iterator<Item = &'a Cell> + 'a {
    //     self.grid
    //     .iter()
    //     .map(|x| self.available_digits_for_cell(x))
    //     .collect();
    // }

    fn used_digits_for_iter<'a, I>(&self, vals: I) -> Option<HashSet<Digit>>
    where
        I: Iterator<Item = &'a Cell>,
    {
        let mut result: HashSet<Digit> = HashSet::new();
        let mut found_more_than_one_digit_in_set: bool = false;
        vals.filter(|x| x.get_value().is_some())
            .collect::<Vec<&Cell>>()
            .iter()
            .for_each(|x| {
                found_more_than_one_digit_in_set |= !result.insert(*x.get_value().as_ref().unwrap())
            });
        if !found_more_than_one_digit_in_set {
            Some(result)
        } else {
            None
        }
    }

    pub fn used_digits_in_row(&self, row: usize) -> Option<HashSet<Digit>> {
        self.used_digits_for_iter(self.row(row))
    }

    pub fn used_digits_in_column(&self, column: usize) -> Option<HashSet<Digit>> {
        self.used_digits_for_iter(self.column(column))
    }

    pub fn used_digits_in_section(&self, section: usize) -> Option<HashSet<Digit>> {
        self.used_digits_for_iter(self.section(section))
    }

    fn unused_digits_for_iter<'a, I>(&self, vals: I) -> Option<HashSet<Digit>>
    where
        I: Iterator<Item = &'a Cell>,
    {
        use super::digit::Digit::*;

        let mut result: HashSet<_> = [One, Two, Three, Four, Five, Six, Seven, Eight, Nine]
            .iter()
            .cloned()
            .collect();
        let mut found_more_than_one_digit_in_set: bool = false;
        vals.filter(|x| x.get_value().is_some())
            .collect::<Vec<&Cell>>()
            .iter()
            .for_each(|x| {
                found_more_than_one_digit_in_set |= !result.remove(&x.get_value().as_ref().unwrap())
            });
        if !found_more_than_one_digit_in_set {
            Some(result)
        } else {
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

    fn get_cell(&self, index: usize) -> Option<&Cell> {
        if index < self.grid_dimensions.get_data_size() {
            Some(&self.grid[index])
        } else {
            None
        }
    }

    fn get_mut_cell(&mut self, index: usize) -> Option<&mut Cell> {
        if index < self.grid_dimensions.get_data_size() {
            Some(&mut self.grid[index])
        } else {
            None
        }
    }

    fn set_digit_for_cell(&mut self, index: usize, digit_value: DigitValue) {
        if index < self.grid_dimensions.get_data_size() {
            self.grid[index].set_value(digit_value);
        }
    }

    fn solve_cell(&self, cell: &Cell) -> DigitValue {
        use super::digit::Digit::*;
        Some(One)
    }

    pub fn solve(&mut self) -> bool {
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
        // find cell with lowest number of posibilities
        false
    }

    pub fn update_column(&mut self, column: usize, values: Vec<Digit>) {
        let mut value_iter = values.iter();
        self.grid_dimensions
            .get_indices_for_column(column)
            .iter()
            .for_each(|i| {
                self.get_mut_cell(*i)
                    .unwrap()
                    .set_value(Some(*value_iter.next().unwrap()))
            });
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
    fn test_unused_digits_in_column() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(
            mysudoku.unused_digits_in_column(3).unwrap().iter().count(),
            9
        );
    }
    #[test]
    fn test_unused_digits_in_section() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(
            mysudoku.unused_digits_in_section(5).unwrap().iter().count(),
            9
        );
    }
    #[test]
    fn test_used_digits_in_row() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(mysudoku.used_digits_in_row(3).unwrap().iter().count(), 0);
    }
    #[test]
    fn test_used_digits_in_column() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(mysudoku.used_digits_in_row(3).unwrap().iter().count(), 0);
    }
    #[test]
    fn test_used_digits_in_section() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(
            mysudoku.used_digits_in_section(3).unwrap().iter().count(),
            0
        );
    }
    #[test]
    fn test_solve_sudoku() {
        let mut mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(mysudoku.solve(), true);
    }
    #[test]
    fn test_update_column() {
        use super::super::digit::Digit::*;

        let mut mysudoku = Sudoku::new(9, 9, 3, 3);
        let row = vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
        mysudoku.update_column(0, row);
        assert_eq!(mysudoku.used_digits_in_column(0).unwrap().iter().count(), 9);
    }
    #[test]
    fn test_used_digits_for_correct_sudoku() {
        use super::super::digit::Digit::*;
        let first_digit_set = vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine];

        let mut mysudoku = Sudoku::new(9, 9, 3, 3);
        let column = vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
        mysudoku.update_column(0, column);
        let column = vec![Four, Five, Six, Seven, Eight, Nine, One, Two, Three];
        mysudoku.update_column(1, column);
        let column = vec![Seven, Eight, Nine, One, Two, Three, Four, Five, Six];
        mysudoku.update_column(2, column);
        let column = vec![Two, Three, Four, Five, Six, Seven, Eight, Nine, One];
        mysudoku.update_column(3, column);
        let column = vec![Five, Six, Seven, Eight, Nine, One, Two, Three, Four];
        mysudoku.update_column(4, column);
        let column = vec![Eight, Nine, One, Two, Three, Four, Five, Six, Seven];
        mysudoku.update_column(5, column);
        let column = vec![Three, Four, Five, Six, Seven, Eight, Nine, One, Two];
        mysudoku.update_column(6, column);
        let column = vec![Six, Seven, Eight, Nine, One, Two, Three, Four, Five];
        mysudoku.update_column(7, column);
        let column = vec![Nine, One, Two, Three, Four, Five, Six, Seven, Eight];
        mysudoku.update_column(8, column);
        assert_eq!(mysudoku.used_digits_in_column(0).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(1).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(2).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(3).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(4).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(5).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(6).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(7).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_column(8).unwrap().iter().count(), 9);
        assert_eq!(
            mysudoku.used_digits_in_section(0).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(1).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(2).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(3).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(4).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(5).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(6).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(7).unwrap().iter().count(),
            9
        );
        assert_eq!(
            mysudoku.used_digits_in_section(8).unwrap().iter().count(),
            9
        );
        assert_eq!(mysudoku.used_digits_in_row(0).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(1).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(2).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(3).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(4).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(5).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(6).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(7).unwrap().iter().count(), 9);
        assert_eq!(mysudoku.used_digits_in_row(8).unwrap().iter().count(), 9);
    }
    #[test]
    fn test_sort_cells_by_freedom() {
        use super::super::digit::Digit::*;

        let mut mysudoku = Sudoku::new(9, 9, 3, 3);
        let row = vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
        mysudoku.update_column(0, row);
        assert_eq!(mysudoku.used_digits_in_column(0).unwrap().iter().count(), 9);
    }
}
