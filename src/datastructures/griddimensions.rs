pub use super::digit::{Digit, DigitValue};

#[derive(Debug, Clone)]
pub struct Cell {
    index: usize,
    column: usize,
    row: usize,
    section: usize,
    value: DigitValue,
}

impl Cell {
    fn new(index: usize, column: usize, row: usize, section: usize, value: DigitValue) -> Self {
        Cell {
            index: index,
            column: column,
            row: row,
            section: section,
            value: value,
        }
    }
    pub fn get_index(&self) -> &usize {
        &self.index
    }
    pub fn get_column(&self) -> &usize {
        &self.column
    }
    pub fn get_row(&self) -> &usize {
        &self.row
    }
    pub fn get_section(&self) -> &usize {
        &self.section
    }
    pub fn get_value(&self) -> &DigitValue {
        &self.value
    }
    pub fn set_value(&mut self, value: DigitValue) {
        self.value = value;
    }
}

#[derive(Debug, Clone)]
pub struct GridDimensions {
    row_count: usize,
    column_count: usize,
    section_width: usize,
    section_height: usize,
}

impl GridDimensions {
    pub fn new(columns: usize, rows: usize, section_width: usize, section_height: usize) -> Self {
        GridDimensions {
            row_count: rows,
            column_count: columns,
            section_width: section_width,
            section_height: section_height,
        }
    }
    pub fn new_grid(&self) -> Vec<Cell> {
        let mut result: Vec<Cell> = Vec::new();
        for column in 0..self.column_count {
            for row in 0..self.row_count {
                let index = column + row * self.column_count;
                result.push(Cell::new(
                    index,
                    column,
                    row,
                    self.get_section_for_position(column, row),
                    None,
                ));
            }
        }
        result
    }
    pub fn is_valid(&self) -> bool {
        (self.row_count % self.section_width == 0) && (self.column_count % self.section_height == 0)
    }
    pub fn get_row_count(&self) -> &usize {
        &self.row_count
    }
    pub fn get_column_count(&self) -> &usize {
        &self.column_count
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
    pub fn get_indices_for_row(&self, row: usize) -> Vec<usize> {
        if row < self.row_count {
            let first = row * self.column_count;
            let last = first + self.column_count;
            (first..last).collect()
        } else {
            Vec::new()
        }
    }
    pub fn get_section_count(&self) -> usize {
        self.get_sections_in_row() * self.get_sections_in_column()
    }
    pub fn get_section_size(&self) -> usize {
        self.section_height * self.section_width
    }
    fn get_index_for_section_index(&self, section: usize, index: usize) -> usize {
        let column_offset = index % self.section_width;
        let row_offset = index / self.section_width;
        let column = (section % self.get_sections_in_row()) * self.section_width + column_offset;
        let row = (section / self.get_sections_in_row()) * self.section_height + row_offset;
        row * self.column_count + column
    }
    pub fn get_indices_for_column(&self, column: usize) -> Vec<usize> {
        if column < self.column_count {
            let first = 0usize;
            let last = self.row_count;
            let range: Vec<usize> = (first..last).collect();
            range.iter().map(|x| x * self.row_count + column).collect()
        } else {
            Vec::new()
        }
    }
    pub fn get_indices_for_section(&self, section: usize) -> Vec<usize> {
        if section < self.get_section_count() {
            let first = 0usize;
            let last = self.get_section_size();
            let range: Vec<usize> = (first..last).collect();
            range
                .iter()
                .map(|x| self.get_index_for_section_index(section, *x))
                .collect()
        } else {
            Vec::new()
        }
    }
    pub fn get_section_for_position(&self, column: usize, row: usize) -> usize {
        row % self.section_width + (column % self.section_height) * self.get_sections_in_row()
    }
    pub fn get_data_size(&self) -> usize {
        self.row_count*self.column_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_of_data() {
        let data = GridDimensions::new(9, 9, 3, 3);
        assert_eq!(data.get_row_count(), &9);
        assert_eq!(data.get_column_count(), &9);
        assert_eq!(data.get_section_height(), &3);
        assert_eq!(data.get_section_width(), &3);
        assert_eq!(data.get_cell_count(), 81);
        assert_eq!(data.get_sections_in_column(), 3);
        assert_eq!(data.get_sections_in_row(), 3);
        assert_eq!(data.get_indices_for_row(3).len(), 9);
        assert_eq!(data.get_indices_for_column(3).len(), 9);
        assert_eq!(data.get_indices_for_section(5).len(), 9);
        assert_eq!(data.get_indices_for_row(3).first(), Some(&27));
        assert_eq!(data.get_indices_for_column(3).first(), Some(&3));
        assert_eq!(data.get_indices_for_section(3).first(), Some(&27));
        assert_eq!(data.get_indices_for_row(5).last(), Some(&53));
        assert_eq!(data.get_indices_for_column(5).last(), Some(&77));
        assert_eq!(data.get_indices_for_section(5).last(), Some(&53));
        assert_eq!(data.get_indices_for_row(10).len(), 0);
        assert_eq!(data.get_indices_for_column(10).len(), 0);
        assert_eq!(data.get_indices_for_section(10).len(), 0);
    }
}
