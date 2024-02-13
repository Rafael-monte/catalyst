use std::{error::Error, fmt::Display};

const ONE: usize = 1;

pub struct CircularList<T> {
    elements: Vec<T>,
    size: usize
}

#[derive(Debug)]
pub struct EmptyListError;

impl Display for EmptyListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Could not remove from an empty list")
    }
}

impl Error for EmptyListError {}


impl<T: Clone> CircularList<T> {

    pub fn new() -> Self {
        return Self {elements: Vec::new(), size: 0}
    }

    pub fn from(v: &[T]) -> Self {
        let size = v.len();
        let elements = v.to_vec();
        return Self {elements, size}
    }

    pub fn push(&mut self, e: T) -> () {
        self.elements.push(e);
        self.size += ONE;
    }

    pub fn remove(&mut self, index: usize) -> Result<(), EmptyListError> {
        if self.elements.is_empty() {
            return Err(EmptyListError);
        }
        self.elements.remove(self.calc_index(index));
        self.size -= ONE;
        return Ok(());
    } 

    pub fn get(&mut self, index: usize) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        }
        let el = self.elements.get(self.calc_index(index)).unwrap().clone();
        return Some(el);
    }

    fn calc_index(&self, idx: usize) -> usize {
        return idx % self.size;
    }

}



mod test_circular_list {
}