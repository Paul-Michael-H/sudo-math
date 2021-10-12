pub use super::data::Data;
pub use super::digit::DigitValue;


pub struct Section<'a> {
    data: &'a Data,
    section: usize,
}

impl<'a> Section<'a> {
    fn get_section_size(&self) -> usize {
        self.data.get_section_width() * self.data.get_section_height()
    }
    pub fn new(data: &'a Data, section: usize) -> Self {
        Section {
            data: data,
            section: section,
        }
    }
    fn get_section_count(&self) -> usize {
        self.data.get_cell_count() / self.get_section_size()        
    }
    fn get_section_top(&self) -> usize {
        (self.section / self.data.get_row_count()) * self.data.get_section_height()
    }
    fn get_section_left(&self) -> usize {
        (self.section % self.data.get_column_count()) * self.data.get_section_width()
    }
}

pub struct SectionIterator<'a> {
    section: Section<'a>,
    current: usize,
}

impl<'a> SectionIterator<'a> {
    fn get_current_position(&self) -> usize {
        let row = self.current / self.section.data.get_section_width();
        let column = self.current % self.section.data.get_section_width();
        (self.section.get_section_top() + column) * self.section.data.get_column_count() + self.section.get_section_left()+row
    }
    pub fn new(section: Section<'a>) -> Self {
        let init_current: usize;
        if section.section < section.get_section_count() {
            init_current = 0;
        } else {
            init_current = section.get_section_size();
        }
        SectionIterator {
            section: section,
            current: init_current,
        }
    }
}

impl <'a> Iterator for SectionIterator<'a> {
    type Item = &'a DigitValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.section.get_section_size() {
            return None
        }
        let position = self.get_current_position();
        self.current += 1;
        Some(&self.section.data.get_value(position))
    }
}
