# Stonks

## Description

This crate provides sets that allow borrowing while inserting entries.

While inserting entries despite referencing other entries in the same set is allowed, removing entries is not possible as it may invalidate references that are held to the entry that is removed.

## Example

### Problematic Code

The following code won't work:

```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::with_capacity(10);
    set.insert("hello");
    let hello = set.get(&"hello").unwrap();

    // Error: Cannot borrow `set` as mutable because it is also borrowed as immutable.
    set.insert("world");

    assert_eq!(hello, &"hello");
}
```

### The Solution

Using Stonks, we can do the following thanks to interior mutability:

```rust
use stonks::Set;

fn main() {
    // Our set doesn't need to be mutable.
    let set = Set::new();
    // Insert some data.
    set.insert("hello");
    // We now have a reference to the data we previously inserted.
    let hello = set.get(&"hello").unwrap();
    // We can insert more data despite holding a reference to it.
    set.insert("world");
    assert_eq!(hello, set.get(&"hello").unwrap());
}
```
