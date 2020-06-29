use std::cell::UnsafeCell;
use std::collections::HashSet;
use std::hash::Hash;

pub struct Set<T>
where
    T: Hash,
{
    data: UnsafeCell<HashSet<Box<T>>>,
}

impl<T> Set<T>
where
    T: Hash + Eq,
{
    /// Create an empty `Set`. Uses a `HashSet` internally.
    pub fn new() -> Self {
        Self {
            data: UnsafeCell::new(HashSet::new()),
        }
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        unsafe {
            (*self.data.get()).len()
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    pub fn get(&self, value: &T) -> Option<&T> {
        unsafe {
            (*self.data.get()).get(value).map(|boxed| boxed.as_ref())
        }
    }

    /// Adds a value to the set.
    /// If the set did not have this value present, `true` is returned.
    /// If the set did have this value present, `false` is returned.
    pub fn insert(&self, value: T) -> bool {
        unsafe {
            (*self.data.get()).insert(Box::new(value))
        }
    }

    /// Inserts the given value into the set if it is not present, then returns a reference to the value in the set.
    /// Currently, this implementation uses requires `T: Clone`, because the feature `hash_set_entry` isn't stabilized yet.
    pub fn get_or_insert(&self, value: T) -> &T
    where
        T: Clone
    {
        if let Some(output) = self.get(&value) {
            output
        } else {
            assert!(self.insert(value.clone()));
            self.get(&value).unwrap()
        }
    }

    /// Returns `true` if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        unsafe {
            (*self.data.get()).contains(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow() {
        let set = Set::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        assert_eq!(set.len(), 5);
    }

    #[test]
    fn insert_while_borrowed() {
        let set = Set::new();
        set.insert(1);
        let first = set.get(&1);
        set.insert(2);
        assert_eq!(*first.unwrap(), 1);
    }

    #[test]
    fn contains_inserted() {
        let set = Set::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.contains(&3), true);
        assert_eq!(set.contains(&4), false);
    }

    #[test]
    fn get_or_insert() {
        let set = Set::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.get_or_insert(3), set.get(&3).unwrap());
        assert_eq!(*set.get_or_insert(4), 4);
    }

    #[test]
    fn big_test() {
        let set = Set::new();
        for i in 0..100000 {
            set.insert(i);
        }
        assert_eq!(set.len(), 100000);

        let mut refs = Vec::new();
        for i in 0..100000 {
            refs.push(set.get(&i).unwrap());
        }

        for i in 100000..200000 {
            set.insert(i);
        }
        assert_eq!(set.len(), 200000);

        for i in 0..100000 {
            assert_eq!(*refs[i], i);
        }
    }
}
