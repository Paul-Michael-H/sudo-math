pub use super::data::Data;
pub use super::digit::DigitValue;

pub struct RowIterator<'a> {
    data: &'a Data,
    row: usize,
    current: usize,
}

impl<'a> RowIterator<'a> {
    fn get_current_position(&self) -> usize {
        (self.row) * self.data.get_column_count() + self.current
    }
    fn at_end(&self) -> bool {
        self.current >= *self.data.get_column_count()
    }
    pub fn new(data: &'a Data, row: usize) -> Self {
        let init_current: usize;
        if row < *data.get_row_count() {
            init_current = 0;
        } else {
            init_current = *data.get_column_count();
        }
        RowIterator {
            data: data,
            row: row,
            current: init_current,
        }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a DigitValue;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.get_current_position();
        if self.at_end() {
            return None;
        }
        self.current += 1;
        Some(&self.data.get_value(position))
    }
}




pub struct RowIteratorMut<'a> {
    data: &'a mut Data,
    row: usize,
    current: usize,
}

impl<'a> RowIteratorMut<'a> {
    fn get_current_position(&self) -> usize {
        (self.row) * self.data.get_column_count() + self.current
    }
    fn at_end(&self) -> bool {
        self.current >= *self.data.get_column_count()
    }
    pub fn new(data: &'a mut Data, row: usize) -> Self {
        let init_current: usize;
        if row < *data.get_row_count() {
            init_current = 0;
        } else {
            init_current = *data.get_column_count();
        }
        RowIteratorMut {
            data: data,
            row: row,
            current: init_current,
        }
    }
}

impl<'a> Iterator for RowIteratorMut<'a> {
    type Item = &'a mut DigitValue;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.get_current_position();
        if self.at_end() {
            return None;
        }
        self.current += 1;
        Some(self.data.get_value(position))
    }
}
