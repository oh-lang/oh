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
#[inline]
pub fn testing_print(bytes: &[u8]) {
    TESTING.with_borrow_mut(|t| {
        t.prints.push(Vec::from(bytes));
    });
}

#[cfg(not(test))]
#[inline]
pub fn testing_print(_bytes: &[u8]) {}

#[cfg(test)]
#[inline]
pub fn testing_unprint() -> Vec<Vec<u8>> {
    TESTING.with_borrow_mut(|t| {
        let mut result = vec![];
        std::mem::swap(&mut result, &mut t.prints);
        result
    })
}

#[cfg(test)]
#[inline]
pub fn testing_name_pointer<T>(pointer: *const T) {
    let pointer = pointer as usize;
    TESTING.with_borrow_mut(|t| {
        if t.pointer_names.contains_key(&pointer) {
            panic!("already created a name for pointer {}", pointer);
        }
        t.pointer_names.insert(pointer, t.next_pointer_name_index);
        t.next_pointer_name_index += 1;
    });
}

#[cfg(not(test))]
#[inline]
pub fn testing_name_pointer<T>(&mut self, _pointer: *const T) {}

#[cfg(test)]
#[inline]
pub fn testing_unname_pointer<T>(pointer: *const T) {
    let pointer = pointer as usize;
    TESTING.with_borrow_mut(|t| {
        let removed = t.pointer_names.remove(&pointer);
        if removed.is_none() {
            panic!(
                "didn't start with a name for pointer {}, call `name_pointer` first",
                pointer
            );
        }
    });
}

#[cfg(not(test))]
#[inline]
pub fn testing_unname_pointer<T>(_pointer: *const T) {}

#[cfg(test)]
#[inline]
pub fn testing_pointer_name<T>(pointer: *const T) -> Vec<u8> {
    let pointer = pointer as usize;
    TESTING.with_borrow(|t| {
        if let Some(name_index) = t.pointer_names.get(&pointer) {
            let name_index = *name_index as u64;
            let abc_index = (name_index % 26) as u8;
            let abc_count = name_index / 26;
            return vec![b'A' + abc_index; 1 + abc_count as usize];
        } else {
            panic!(
                "pointer {} does not have a name yet, call `name_pointer` first",
                pointer
            );
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn printing_works_thread_a() {
        testing_print(b"this is a string");
        testing_print(b"yet another string");

        assert_eq!(
            testing_unprint(),
            vec![
                Vec::from(b"this is a string"),
                Vec::from(b"yet another string")
            ]
        );

        testing_print(b"afterwards");
        assert_eq!(testing_unprint(), vec![b"afterwards"],);
    }

    #[test]
    fn printing_works_thread_b() {
        testing_print(b"this is a string in another thread");
        testing_print(b"yet another string in another thread");

        assert_eq!(
            testing_unprint(),
            vec![
                Vec::from(b"this is a string in another thread"),
                Vec::from(b"yet another string in another thread")
            ]
        );
        assert_eq!(testing_unprint().len(), 0);
    }

    #[test]
    fn naming_pointers_works() {
        let x: u8 = 13;
        let y: u16 = 15;
        let z: u32 = 17;
        testing_name_pointer(&x);
        assert_eq!(testing_pointer_name(&x), Vec::from(b"A"));
        testing_name_pointer(&y);
        assert_eq!(testing_pointer_name(&y), Vec::from(b"B"));
        // Don't re-use pointer names after unnaming:
        testing_unname_pointer(&x);
        testing_name_pointer(&z);
        assert_eq!(testing_pointer_name(&z), Vec::from(b"C"));
    }

    #[test]
    #[should_panic]
    fn getting_pointer_names_fails_without_first_naming() {
        let x: u8 = 13;
        testing_pointer_name(&x);
    }

    #[test]
    #[should_panic]
    fn unname_pointer_fails_without_first_naming() {
        let x: u8 = 13;
        testing_unname_pointer(&x);
    }
}
