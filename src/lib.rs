use std::{
    alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout},
    fmt::Debug,
};

#[derive(Debug, PartialEq)]
pub struct Hallo {
    value: u32,
    extra: u8,
}

#[derive(Debug, PartialEq)]
pub struct Arr<T> {
    next: *mut Arr<T>,
    data: Option<T>,
}

impl<T> std::default::Default for Arr<T> {
    fn default() -> Self {
        Arr {
            next: std::ptr::null_mut(),
            data: None,
        }
    }
}

impl<T: Debug> Arr<T> {
    pub fn new(data: T) -> Self {
        Arr {
            next: std::ptr::null_mut(),
            data: Some(data),
        }
    }

    pub fn append(&mut self, item: T) {
        let new = Arr::new(item);
        let new_ptr = Box::into_raw(Box::new(new));
        let mut current = self;
        while !current.next.is_null() {
            unsafe {
                current = &mut *current.next;
            }
        }
        current.next = new_ptr;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut current = self;
        let mut previous = None;
        let mut index = 0;
        while !current.next.is_null() {
            dbg!(&current);
            if index > 10 {
                panic!()
            }
            unsafe {
                previous = Some(std::ptr::from_mut(current));
                current = &mut *current.next;
            }
            index += 1;
        }

        if let Some(previous) = previous {
            let now = unsafe { &mut *previous };
            unsafe {
                let found = std::ptr::replace(now.next, Arr::default());
                now.next = std::ptr::null_mut();
                return Some(found.data.expect("data is not none"));
            }
        }

        current.data.take()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = self;
        for _ in 0..index {
            if current.next.is_null() {
                return None;
            }
            unsafe {
                current = &*(current.next);
            }
        }

        current.data.as_ref()
    }

    pub fn replace(&mut self, index: usize, item: T) -> Option<T> {
        let mut current = self;
        for _ in 0..index {
            if current.next.is_null() {
                return None;
            }
            unsafe {
                current = &mut *(current.next);
            }
        }

        unsafe {
            let new = Arr {
                data: Some(item),
                next: (*current.next).next,
            };

            std::ptr::replace(current.next, new).data
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index == 0 {
            let mut new = Arr::new(item);
            let new_self = Arr {
                data: self.data.take(),
                next: self.next
            };
            let new_ptr = Box::into_raw(Box::new(new_self));
            new.next = new_ptr;
            *self = new;
            return
        }
        let mut current = self;
        for _ in 1..index {
            if current.next.is_null() {
                // error?
                return;
            }
            unsafe {
                current = &mut *(current.next);
            }
        }

        if current.next.is_null() {
            let new = Self::new(item);
            let new_ptr = Box::into_raw(Box::new(new));
            current.next = new_ptr;
        } else {
            let new = Arr {
                data: Some(item),
                next: current.next,
            };

            let new_ptr = Box::into_raw(Box::new(new));
            current.next = new_ptr;
        }
    }

    pub fn to_vec(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut index = 0;
        while let Some(item) = self.pop() {
            vec.push(item);

        }
        vec.reverse();
        vec
    }
}

// pub fn start() -> bool {
//     unsafe {
//         let layout = Layout::new::<Hallo>();
//         let ptr = alloc_zeroed(layout);
//         if ptr.is_null() {
//             handle_alloc_error(layout);
//         }

//         let value_ptr = ptr as *mut Hallo;
//         let val: &mut Hallo = &mut *value_ptr;
//         val.value = 42;

//         let out = (*value_ptr) == Hallo{value: 42, extra: 0};

//         dealloc(ptr, layout);

//         dbg!(&*value_ptr);

//         out
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert!(start());
//     }
// }

#[test]
fn arr_test() {
    let mut a = Arr::new(1);
    a.append(2);
    a.append(3);
    a.append(4);
    a.append(5);

    assert_eq!(a.pop(), Some(5));
    assert_eq!(a.get(4), None);
    assert_eq!(a.get(1), Some(&2));
    a.append(6);
    dbg!(&(std::ptr::addr_of!(a)));
    a.insert(0, 9);
    dbg!(&(std::ptr::addr_of!(a)));
    dbg!("xd");
    a.insert(1, 1);
    dbg!("xp");
    a.insert(3, 3);
    dbg!("xc");
    assert_eq!(a.to_vec(), vec![9, 1, 1, 3, 2, 3, 4, 6]);
}
