use stonks::StonksSet;

fn main() {
    // Our set doesn't need to be mutable.
    let set = StonksSet::with_capacity(10);
    // Insert some data.
    set.insert("hello");
    // We now have a refefence to the data we previously inserted.
    let hello = set.get(&"hello").unwrap();
    // We can insert more data despite holding a reference to it.
    set.insert("world");
    assert_eq!(hello, set.get(&"hello").unwrap());
}