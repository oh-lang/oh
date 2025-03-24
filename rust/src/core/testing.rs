#![allow(dead_code)]
use crate::core::count::*;
use crate::core::signed::*;

use std::cell::RefCell;
use std::collections::hash_map::HashMap;

thread_local! {
    /// use `TESTING_DATA.with_borrow_mut(|t| t.print(b"whatever"))` to add debug things to test in unit tests, etc.
    static TESTING_DATA: RefCell<TestingData> = RefCell::new(TestingData::default());
}

struct TestingData {
    prints: Vec<Vec<u8>>,
    pointer_names: HashMap<usize, u64>,
    next_pointer_name_index: u64,
}

impl Default for TestingData {
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
    TESTING_DATA.with_borrow_mut(|t| {
        t.prints.push(Vec::from(bytes));
    });
}

#[cfg(not(test))]
#[inline]
pub fn testing_print(_bytes: &[u8]) {}

#[cfg(test)]
#[inline]
pub fn testing_unprint(mut expected_values: Vec<Vec<u8>>) {
    let mut actual_values = TESTING_DATA.with_borrow_mut(|t| {
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
            eprint!("{:02} ", i);
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
                eprint!("[actual]: {}    <->", left);
            } else {
                eprint!("[actual]: {:?}    <->", left);
            }
            if let Ok(right) = std::str::from_utf8(&right) {
                eprintln!("    [expected]: {}", right);
            } else {
                eprintln!("    [expected]: {:?}", right);
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

pub enum TestingPointer<S: SignedPrimitive, T> {
    One(*const T),
    Count(*const T, Count<S>),
}

pub fn testing_pointer_one<T>(pointer: *const T) -> TestingPointer<i64, T> {
    TestingPointer::One(pointer)
}

impl<S: SignedPrimitive, T> TestingPointer<S, T> {
    pub fn pointer(&self) -> *const T {
        match self {
            TestingPointer::One(pointer) => *pointer,
            TestingPointer::Count(pointer, _) => *pointer,
        }
    }

    pub fn name_label(&self, mut pointer_name: Vec<u8>) -> Vec<u8> {
        match self {
            TestingPointer::One(_) => {}
            TestingPointer::Count(_, count) => {
                let string = if count.is_null() {
                    ": Null".to_string()
                } else {
                    format!(": {}", count.to_usize())
                };
                pointer_name.append(&mut Vec::from(<String as AsRef<[u8]>>::as_ref(&string)));
            }
        }
        pointer_name
    }
}

#[cfg(test)]
#[inline]
pub fn testing_name_pointer<S: SignedPrimitive, T>(what: TestingPointer<S, T>) {
    let pointer = what.pointer() as usize;
    TESTING_DATA.with_borrow_mut(|t| {
        if t.pointer_names.contains_key(&pointer) {
            panic!("already created a name for pointer {}", pointer);
        }
        let name_index = t.next_pointer_name_index;
        t.pointer_names.insert(pointer, name_index);
        t.next_pointer_name_index += 1;
        let mut print = Vec::from(b"create(");
        print.append(&mut what.name_label(u64_name(name_index)));
        print.push(b')');
        t.prints.push(print);
    });
}

#[cfg(not(test))]
#[inline]
pub fn testing_name_pointer<S: SignedPrimitive, T>(_what: TestingPointer<S, T>) {}

#[cfg(test)]
#[inline]
pub fn testing_unname_pointer<T>(pointer: *const T) {
    let pointer = pointer as usize;
    TESTING_DATA.with_borrow_mut(|t| {
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
    TESTING_DATA.with_borrow(|t| {
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
        testing_name_pointer(testing_pointer_one(&x));
        assert_eq!(testing_pointer_name(&x), Vec::from(b"A"));
        testing_name_pointer(testing_pointer_one(&y));
        assert_eq!(testing_pointer_name(&y), Vec::from(b"B"));
        // Don't re-use pointer names after unnaming:
        testing_unname_pointer(&x);
        // This is not actually an array of 23 elements, but neither are `&x` and `&y` allocked pointers.
        testing_name_pointer(TestingPointer::Count(&z, Count8::of(23).expect("ok")));
        assert_eq!(testing_pointer_name(&z), Vec::from(b"C"));
        testing_unprint(vec![
            Vec::from(b"create(A)"),
            Vec::from(b"create(B)"),
            Vec::from(b"delete(A)"),
            Vec::from(b"create(C: 23)"),
        ]);
    }

    #[test]
    #[should_panic]
    fn unprint_can_fail() {
        let w: u8 = 20;
        testing_name_pointer(testing_pointer_one(&w));
        testing_unprint(vec![Vec::from(b"delete(A)")]);
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
