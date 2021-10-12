pub use super::data::Data;
pub use super::digit::DigitValue;

pub struct ColumnIterator<'a> {
    data: &'a Data,
    column: usize,
    current: usize,
}

impl<'a> ColumnIterator<'a> {
    fn get_current_position(&self) -> usize {
        self.current * self.data.get_row_count() + self.column
    }
    fn at_end(&self) -> bool {
        self.current >= *self.data.get_row_count()
    }
    pub fn new(data: &'a Data, column: usize) -> Self {
        let init_current: usize;
        if column < *data.get_column_count() {
            init_current = 0;
        } else {
            init_current = *data.get_row_count();
        }
        ColumnIterator {
            data: data,
            column: column,
            current: init_current,
        }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = &'a DigitValue;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.get_current_position();
        if self.at_end() {
            return None;
        }
        self.current += 1;
        Some(self.data.get_value(position))
    }
}
