use crate::StonksVec;
use std::cell::UnsafeCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct StonksSet<T>
where
    T: Hash,
{
    data: UnsafeCell<Vec<StonksVec<T>>>,
}

impl<T> StonksSet<T>
where
    T: Hash,
{
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0);

        Self {
            data: {
                let mut vec = Vec::with_capacity(capacity);
                for _ in 0..capacity {
                    vec.push(StonksVec::new());
                }
                UnsafeCell::new(vec)
            },
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.data.get()).iter().map(|bucket| bucket.len()).sum() }
    }

    fn get_bucket(&self, value: &T) -> &StonksVec<T> {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        unsafe {
            let index = hasher.finish() as usize % (*self.data.get()).len();
            &(*self.data.get())[index]
        }
    }

    pub fn get(&self, value: &T) -> Option<&T>
    where
        T: PartialEq,
    {
        self.get_bucket(value).get(value)
    }

    pub fn insert(&self, value: T) -> &T {
        self.get_bucket(&value).insert(value)
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
        self.get_bucket(&value).contains(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow() {
        let stonks = StonksSet::with_capacity(2);
        stonks.insert(1);
        stonks.insert(2);
        stonks.insert(3);
        stonks.insert(4);
        stonks.insert(5);
        assert_eq!(stonks.len(), 5);
    }

    #[test]
    fn insert_while_borrowed() {
        let stonks = StonksSet::with_capacity(10);
        stonks.insert(1);
        let first = stonks.get(&1);
        stonks.insert(2);
        assert_eq!(*first.unwrap(), 1);
    }

    #[test]
    fn contains_inserted() {
        let stonks = StonksSet::with_capacity(10);
        stonks.insert(1);
        stonks.insert(2);
        stonks.insert(3);
        assert_eq!(stonks.contains(3), true);
        assert_eq!(stonks.contains(4), false);
    }

    #[test]
    fn get_or_insert() {
        let stonks = StonksSet::with_capacity(10);
        stonks.insert(1);
        stonks.insert(2);
        stonks.insert(3);
        assert_eq!(stonks.get_or_insert(3), stonks.get(&3).unwrap());
        assert_eq!(*stonks.get_or_insert(4), 4);
    }
}
