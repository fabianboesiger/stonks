use std::cell::UnsafeCell;

pub struct StonksVec<T> {
    data: UnsafeCell<Vec<T>>,
}

impl<T> StonksVec<T> {
    pub fn new() -> Self {
        Self {
            data: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: UnsafeCell::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.data.get()).len() }
    }

    pub fn at(&self, index: usize) -> &T {
        unsafe {
            assert!(index < (*self.data.get()).len());

            &*(*self.data.get()).as_ptr().add(index)
        }
    }

    fn end(&self) -> *const T {
        unsafe { (*self.data.get()).as_ptr().add((*self.data.get()).len()) }
    }

    pub fn get(&self, value: &T) -> Option<&T>
    where
        T: PartialEq,
    {
        unsafe {
            let mut ptr = (*self.data.get()).as_ptr();
            let end = self.end();
            while ptr != end {
                if *ptr == *value {
                    return Some(&*ptr);
                }
                ptr = ptr.add(1);
            }
        }

        None
    }

    pub fn insert(&self, value: T) -> &T {
        unsafe {
            (*self.data.get()).push(value);
            let end = self.end().offset(-1);
            &*end
        }
    }

    pub fn get_or_insert(&self, value: T) -> &T
    where
        T: PartialEq,
    {
        if let Some(output) = self.get(&value) {
            output
        } else {
            self.insert(value)
        }
    }

    pub fn contains(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        unsafe {
            for element in &*self.data.get() {
                if *element == value {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let stonks = StonksVec::new();
        stonks.insert(1);
        stonks.insert(2);
        stonks.insert(3);
        assert_eq!(stonks.len(), 3);
    }

    #[test]
    fn at() {
        let stonks = StonksVec::new();
        stonks.insert(1);
        stonks.insert(2);
        let first = stonks.at(1);
        assert_eq!(*first, 2);
    }

    #[test]
    fn insert_while_borrowed() {
        let stonks = StonksVec::new();
        stonks.insert(1);
        let first = stonks.get(&1);
        stonks.insert(2);
        assert_eq!(*first.unwrap(), 1);
    }

    #[test]
    fn contains_inserted() {
        let stonks = StonksVec::new();
        stonks.insert(1);
        stonks.insert(2);
        stonks.insert(3);
        assert_eq!(stonks.contains(3), true);
        assert_eq!(stonks.contains(4), false);
    }

    #[test]
    fn get_or_insert() {
        let stonks = StonksVec::new();
        stonks.insert(1);
        stonks.insert(2);
        stonks.insert(3);
        assert_eq!(stonks.get_or_insert(3), stonks.get(&3).unwrap());
        assert_eq!(*stonks.get_or_insert(4), 4);
    }
}
