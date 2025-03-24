#![allow(dead_code)]
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

thread_local! {
    /// use `TESTING.with_borrow_mut(|t| t.print(b"whatever"))` to add debug things to test in unit tests, etc.
    pub static TESTING: RefCell<Testing> = RefCell::new(Testing::default());
}

struct Testing {
    prints: Vec<Vec<u8>>,
    pointer_names: HashMap<usize, u64>,
    next_pointer_name_index: u64,
}

impl Default for Testing {
    fn default() -> Self {
        Self {
            prints: Default::default(),
            pointer_names: Default::default(),
            next_pointer_name_index: 0,
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

    #[inline]
    pub fn name_pointer<T>(&mut self, pointer: *mut T) {
        let pointer = pointer as usize;
        if self.pointer_names.contains_key(&pointer) {
            panic!("already created a name for pointer {}", pointer);
        }
        self.pointer_names
            .insert(pointer, self.next_pointer_name_index);
        self.next_pointer_name_index += 1;
    }

    #[inline]
    pub fn unname_pointer<T>(&mut self, pointer: *mut T) {
        let pointer = pointer as usize;
        let removed = self.pointer_names.remove(&pointer);
        if removed.is_none() {
            panic!(
                "didn't start with a name for pointer {}, call `name_pointer` first",
                pointer
            );
        }
    }

    #[inline]
    pub fn pointer_name<T>(&self, pointer: *mut T) -> Vec<u8> {
        let pointer = pointer as usize;
        if let Some(name_index) = self.pointer_names.get(&pointer) {
            let name_index = *name_index as u64;
            let abc_index = (name_index % 26) as u8;
            let abc_count = name_index / 26;
            return vec![b'A' + abc_index; abc_count as usize];
        } else {
            panic!(
                "pointer {} does not have a name yet, call `name_pointer` first",
                pointer
            );
        }
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

    #[inline]
    pub fn name_pointer<T>(&mut self, _pointer: *mut T) {}

    #[inline]
    pub fn unname_pointer<T>(&mut self, _pointer: *mut T) {}

    #[inline]
    pub fn pointer_name<T>(&self, _pointer: *mut T) -> Vec<u8> {
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

        TESTING.with_borrow_mut(|t| t.print(b"afterwards"));
        assert_eq!(
            TESTING.with_borrow_mut(|t| t.unprint()),
            vec![b"afterwards"],
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
        assert_eq!(TESTING.with_borrow_mut(|t| t.unprint()).len(), 0);
    }
}
