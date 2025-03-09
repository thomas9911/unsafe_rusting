use std::{
    // alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout},
    fmt::Debug,
};

// #[derive(Debug, PartialEq)]
// pub struct Hallo {
//     value: u32,
//     extra: u8,
// }

#[derive(Debug, PartialEq)]
pub struct Arr<T> {
    next: *mut Arr<T>,
    data: Option<T>,
}

impl<T> std::default::Default for Arr<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Arr<T> {
    pub fn new(data: T) -> Self {
        Arr {
            next: std::ptr::null_mut(),
            data: Some(data),
        }
    }

    pub fn empty() -> Self {
        Arr {
            next: std::ptr::null_mut(),
            data: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        debug_assert!(self.next.is_null());
        self.data.is_none()
    }

    pub fn append(&mut self, item: T) {
        self.inner_append(item);
    }

    fn inner_append(&mut self, item: T) -> &mut Self {
        let new = Arr::new(item);
        let new_ptr = Box::into_raw(Box::new(new));
        let mut current = self;
        while !current.next.is_null() {
            dbg!(&current.next);
            unsafe {
                current = &mut *current.next;
            }
        }
        current.next = new_ptr;
        return current;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut current = self;
        let mut previous = None;
        while !current.next.is_null() {
            unsafe {
                previous = Some(std::ptr::from_mut(current));
                current = &mut *current.next;
            }
        }

        if let Some(previous) = previous {
            let now = unsafe { &mut *previous };
            unsafe {
                let found = std::ptr::replace(now.next, Arr::empty());
                now.next = std::ptr::null_mut();
                return found.data;
            }
        }

        current.data.take()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.next.is_null() {
            return self.data.take();
        }

        let item = self.data.take();
        unsafe {
            let new_self = std::ptr::read(self.next);
            *self = new_self;
        }
        item
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

    pub fn push(&mut self, item: T) {
        let mut new = Arr::new(item);
        let new_self = Arr {
            data: self.data.take(),
            next: self.next,
        };
        let new_ptr = Box::into_raw(Box::new(new_self));
        new.next = new_ptr;
        *self = new;
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index == 0 {
            return self.push(item);
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

    pub fn to_vec(self) -> Vec<T> {
        self.into_iter().collect()
    }

    pub fn iter<'a>(&'a self) -> IterRef<'a, T> {
        IterRef {
            arr: self,
            current_index: 0,
        }
    }
}

impl<A> std::iter::FromIterator<A> for Arr<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();
        if let Some(first) = iterator.next() {
            let mut arr = Arr::new(first);
            let mut current = &mut arr;
            for item in iterator {
                // to make it more efficient, because we know where the end will be.
                //
                // normal append loops over the next until the end
                //
                // inner_append returns the last item added,
                // so we don't need to loop to the end, we are already there
                current = current.inner_append(item);
            }
            return arr;
        } else {
            return Arr::empty();
        }
    }
}

impl<T> std::iter::IntoIterator for Arr<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { arr: self }
    }
}

pub struct Iter<T> {
    arr: Arr<T>,
}

impl<T> std::iter::Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.arr.pop_front()
    }
}

pub struct IterRef<'a, T> {
    arr: &'a Arr<T>,
    current_index: usize,
}

impl<'a, T> std::iter::Iterator for IterRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.arr.get(self.current_index);
        self.current_index += 1;
        item
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
    a.insert(0, 9);
    a.push(8);
    a.insert(1, 1);
    a.insert(3, 3);

    let mut items = Vec::new();

    for item in a.iter() {
        items.push(item);
    }

    assert_eq!(items, vec![&8, &1, &9, &3, &1, &2, &3, &4, &6]);
    assert_eq!(a.to_vec(), vec![8, 1, 9, 3, 1, 2, 3, 4, 6]);
}

#[test]
fn arr_test_from_vec() {
    let data = vec![1, 2, 3, 4, 5];
    let a = Arr::from_iter(data.clone());

    assert_eq!(a.to_vec(), data);
}
