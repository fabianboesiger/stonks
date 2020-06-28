use std::cell::UnsafeCell;

pub struct StonksVec<T> {
    data: UnsafeCell<Vec<Box<T>>>,
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

            (*self.data.get())[index].as_ref()
        }
    }

    pub fn get(&self, value: &T) -> Option<&T>
    where
        T: PartialEq,
    {
        for element in unsafe { &*self.data.get() } {
            if **element == *value {
                return Some(element.as_ref());
            }
        }

        None
    }

    pub fn insert(&self, value: T) -> &T {
        let boxed = Box::new(value);
        unsafe {
            (*self.data.get()).push(boxed);
            (*self.data.get()).last().unwrap().as_ref()
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

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        unsafe {
            for element in &*self.data.get() {
                if **element == *value {
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
        let set = StonksVec::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn at() {
        let set = StonksVec::new();
        set.insert(1);
        set.insert(2);
        let first = set.at(1);
        assert_eq!(*first, 2);
    }

    #[test]
    fn insert_while_borrowed() {
        let set = StonksVec::new();
        set.insert(1);
        let first = set.get(&1);
        set.insert(2);
        assert_eq!(*first.unwrap(), 1);
    }

    #[test]
    fn contains_inserted() {
        let set = StonksVec::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.contains(&3), true);
        assert_eq!(set.contains(&4), false);
    }

    #[test]
    fn get_or_insert() {
        let set = StonksVec::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.get_or_insert(3), set.get(&3).unwrap());
        assert_eq!(*set.get_or_insert(4), 4);
    }

    #[test]
    fn big_test() {
        let set = StonksVec::new();
        for i in 0..10000 {
            set.insert(i);
        }
        assert_eq!(set.len(), 10000);

        let mut refs = Vec::new();
        for i in 0..10000 {
            refs.push(set.get(&i).unwrap());
        }

        for i in 10000..20000 {
            set.insert(i);
        }
        assert_eq!(set.len(), 20000);

        for i in 0..10000 {
            assert_eq!(*refs[i], i);
        }
    }
}
