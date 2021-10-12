pub use super::digit::{Digit, DigitValue};

#[derive(Debug, Clone)]
pub struct Data {
    data: Vec<DigitValue>,
    row_count: usize,
    column_count: usize,
    section_width: usize,
    section_height: usize,
}

impl Data {
    pub fn new(columns: usize, rows: usize, section_width: usize, section_height: usize) -> Self {
        let size = rows * columns;
        let value: DigitValue = None;
        Data {
            data: std::iter::repeat(value).take(size).collect::<Vec<_>>(),
            row_count: rows,
            column_count: columns,
            section_width: section_width,
            section_height: section_height,
        }
    }
    pub fn get_row_count(&self) -> &usize {
        &self.row_count
    }
    pub fn get_column_count(&self) -> &usize {
        &self.column_count
    }
    pub fn get_value(&self, position: usize) -> &DigitValue {
        &self.data[position]
    }
    pub fn get_value_mut(&mut self, position: usize) -> &mut DigitValue {
        &mut self.data[position]
    }
    pub fn get_section_height(&self) -> &usize {
        &self.section_height
    }
    pub fn get_section_width(&self) -> &usize {
        &self.section_width
    }
    pub fn get_cell_count(&self) -> usize {
        self.row_count * self.column_count
    }
    pub fn get_sections_in_row(&self) -> usize {
        self.row_count / self.section_width
    }
    pub fn get_sections_in_column(&self) -> usize {
        self.column_count / self.section_height
    }
}
