#![allow(dead_code)]
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

thread_local! {
    /// use `TESTING.with_borrow_mut(|t| t.print(b"whatever"))` to add debug things to test in unit tests, etc.
    pub static TESTING: RefCell<Testing> = RefCell::new(Testing::default());
}

struct Testing {
    prints: Vec<Vec<u8>>,
    pointer_names: HashMap<u64, String>,
    next_pointer_name: String,
}

impl Default for Testing {
    fn default() -> Self {
        Self {
            prints: Default::default(),
            pointer_names: Default::default(),
            next_pointer_name: "A".into(),
        }
    }
}

#[cfg(test)]
impl Testing {
    #[inline]
    pub fn print(&mut self, bytes: &[u8]) {
        self.prints.push(Vec::from(bytes));
    }

    #[inline]
    pub fn unprint(&mut self) -> Vec<Vec<u8>> {
        let mut result = vec![];
        std::mem::swap(&mut result, &mut self.prints);
        result
    }
}

#[cfg(not(test))]
impl Testing {
    #[inline]
    pub fn print(&mut self, _bytes: &[u8]) {}

    #[inline]
    pub fn unprint(&mut self) -> Vec<Vec<u8>> {
        panic!("test only");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn printing_works_thread_a() {
        TESTING.with_borrow_mut(|t| t.print(b"this is a string"));
        TESTING.with_borrow_mut(|t| t.print(b"yet another string"));

        assert_eq!(
            TESTING.with_borrow_mut(|t| t.unprint()),
            vec![
                Vec::from(b"this is a string"),
                Vec::from(b"yet another string")
            ]
        );
    }

    #[test]
    fn printing_works_thread_b() {
        TESTING.with_borrow_mut(|t| t.print(b"this is a string in another thread"));
        TESTING.with_borrow_mut(|t| t.print(b"yet another string in another thread"));

        assert_eq!(
            TESTING.with_borrow_mut(|t| t.unprint()),
            vec![
                Vec::from(b"this is a string in another thread"),
                Vec::from(b"yet another string in another thread")
            ]
        );
    }
}
