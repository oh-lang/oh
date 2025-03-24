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
pub fn testing_unprint(mut expected_values: Vec<Vec<u8>>) {
    let mut actual_values = TESTING.with_borrow_mut(|t| {
        let mut result = vec![];
        std::mem::swap(&mut result, &mut t.prints);
        result
    });
    let same_len = actual_values.len() == expected_values.len();
    let mut first_bad_i: i64 = -1;
    for i in 0..actual_values.len().min(expected_values.len()) {
        if actual_values[i] != expected_values[i] {
            first_bad_i = i as i64;
            break;
        }
    }
    if !same_len || first_bad_i >= 0 {
        eprintln!("left (actual) was not equal to right (expected)");
        for i in 0..actual_values.len().max(expected_values.len()) {
            eprint!("{:02}: ", i);
            let left = if i < actual_values.len() {
                moot(&mut actual_values[i])
            } else {
                Vec::from(b"{missing}")
            };
            let right = if i < expected_values.len() {
                moot(&mut expected_values[i])
            } else {
                Vec::from(b"{missing}")
            };
            if let Ok(left) = std::str::from_utf8(&left) {
                eprint!("[a] {}    <-> ", left);
            } else {
                eprint!("[a] {:?}    <-> ", left);
            }
            if let Ok(right) = std::str::from_utf8(&right) {
                eprintln!("[e] {}", right);
            } else {
                eprintln!("[e] {:?}", right);
            }
        }
        if first_bad_i >= 0 {
            eprintln!("first different at index {}", first_bad_i);
        }
        panic!("left (actual) was not equal to right (expected)");
    }
}

fn moot<T: Default>(t: &mut T) -> T {
    let mut result = T::default();
    std::mem::swap(&mut result, t);
    result
}

#[cfg(test)]
#[inline]
pub fn testing_name_pointer<T>(pointer: *const T) {
    let pointer = pointer as usize;
    TESTING.with_borrow_mut(|t| {
        if t.pointer_names.contains_key(&pointer) {
            panic!("already created a name for pointer {}", pointer);
        }
        let name_index = t.next_pointer_name_index;
        t.pointer_names.insert(pointer, name_index);
        t.next_pointer_name_index += 1;
        let mut print = Vec::from(b"create(");
        print.append(&mut u64_name(name_index));
        print.push(b')');
        t.prints.push(print);
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
        if let Some(removed_index) = t.pointer_names.remove(&pointer) {
            let mut print = Vec::from(b"delete(");
            print.append(&mut u64_name(removed_index));
            print.push(b')');
            t.prints.push(print);
        } else {
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
            u64_name(*name_index)
        } else {
            panic!(
                "pointer {} does not have a name yet, call `name_pointer` first",
                pointer
            );
        }
    })
}

fn u64_name(name_index: u64) -> Vec<u8> {
    let abc_index = (name_index % 26) as u8;
    let abc_count = name_index / 26;
    return vec![b'A' + abc_index; 1 + abc_count as usize];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn printing_works_thread_a() {
        testing_print(b"this is a string");
        testing_print(b"yet another string");

        testing_unprint(vec![
            Vec::from(b"this is a string"),
            Vec::from(b"yet another string"),
        ]);

        testing_print(b"afterwards");
        testing_unprint(vec![Vec::from(b"afterwards")]);
    }

    #[test]
    fn printing_works_thread_b() {
        testing_print(b"this is a string in another thread");
        testing_print(b"yet another string in another thread");

        testing_unprint(vec![
            Vec::from(b"this is a string in another thread"),
            Vec::from(b"yet another string in another thread"),
        ]);
        testing_unprint(vec![]);
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
        testing_unprint(vec![
            Vec::from(b"create(A)"),
            Vec::from(b"create(B)"),
            Vec::from(b"delete(B)"),
            Vec::from(b"create(C)"),
        ]);
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
