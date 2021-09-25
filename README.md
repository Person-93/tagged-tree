# Tagged Tree

This crate provides a simple tree structure in which all the nodes (except the root)
are tagged with an arbitrary type.

## Example

```rust
use tagged_tree::Tree;

fn main() {
    // create the tree with the root node
    let mut tree = Tree::<&'static str, i32>::new(42);
    assert_eq!(*tree.value(), 42);

    // add the value 1 tagged with "hello"
    let (old_val, child) = tree.add_child("hello", 1);
    assert!(old_val.is_none());
    assert_eq!(*child.value(), 1);

    // replace it with 5
    let (old_val, child) = tree.add_child("hello", 5);
    assert_eq!(old_val.unwrap(), 1);
    assert_eq!(*child.value(), 5);

    // add a child to the child
    child.add_child("world", 2);

    // add another child to the original
    tree.add_child("greetings", 3);

    // iterate over the direct children
    for (key, child) in tree.iter_single() {
        println!("{}: {}", key, child.value());
    }

    // depth first traversal
    for (key, value) in tree.iter_depth_first() {
        println!("{}: {}", key, value);
    }

    // breadth first traversal
    for (key, value) in tree.iter_breadth_first() {
        println!("{}: {}", key, value);
    }
}
```
