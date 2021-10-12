// how to implement a set?
// a set is a non organised collection of items on which certain operations can take place
use super::digit::DigitValue;
use super::data::Data;
use super::section::{SectionIterator,Section};
use super::row::{RowIterator,RowIteratorMut};
use super::column::ColumnIterator; 

pub struct Sudoku {
    data: Data,
}

impl Sudoku {
    pub fn new(columns: usize, rows: usize, section_width: usize, section_height: usize) -> Self {
        Sudoku {
            data: Data::new(columns, rows, section_width, section_height),
        }
    }

    pub fn row<'a>(&'a self, row: usize) -> impl Iterator<Item = &DigitValue> + 'a {
        RowIterator::new(&self.data, row)
    }

    pub fn row_mut<'a>(&'a mut self, row: usize) -> impl Iterator<Item = &mut DigitValue> + 'a {
        RowIteratorMut::new(&mut self.data, row)
    }

    pub fn column<'a>(&'a self, column: usize) -> impl Iterator<Item = &DigitValue> + 'a {
        ColumnIterator::new(&self.data, column)
    }

    pub fn section<'a>(&'a self, section: usize) -> impl Iterator<Item = &DigitValue> + 'a {
        let section = Section::new(&self.data, section);
        SectionIterator::new(section)
    }
    
    pub fn is_valid(&self) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_make_an_empty_sudoku() {
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        assert_eq!(mysudoku.is_valid(), false);
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
   fn test_update_cell() {
        use super::super::digit::Digit::*;
        let mysudoku = Sudoku::new(9, 9, 3, 3);
        let column = mysudoku.row_mut(0).for_each(|x| *x = Some(One));
        assert_eq!(9, mysudoku.row(0).filter(|x| *x == &Some(One)).count());
  }


}
