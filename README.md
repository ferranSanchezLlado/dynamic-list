# Dynamic List 

A versatile implementation of a dynamic bidirectional linked list in Rust, capable of storing any type. This fully-typed list eliminates the need for extra costs, such as dynamic dispatching, making it efficient and flexible for a wide range of use cases.

## Installation 🚀

Add the following to your `Cargo.toml`:

```toml
[dependencies]
dynamic-list = "0.2.0"
```

Or if you want to use the latest version from the master branch:

```toml
[dependencies]
dynamic-list = { git = "https://github.com/ferranSanchezLlado/dynamic-list.git" }
```

## Usage 🛠️

The main way to interact with the `DynamicList` is through the implementation of a trait that allows it to iterate, reduce, or accumulate by recursively calling itself.

Example 1: Get a specific element

```rust
use dynamic_list::*;
use typenum::*;

let list = list![1u8, "hello", true, "world"];
assert_eq!(Index::<U1>::index(list_1.forward()), &"hello");
assert_eq!(Index::<U3>::index(list_1.forward()), &"world");

```

Example 2: We want to concatenate a list of items into a single string:

```rust
use dynamic_list::*;

// Iterator trait
trait Concat {
    fn concat(&self) -> String;
}
impl<V: ToString + Clone> Concat for Node<V> {
    fn concat(&self) -> String {
        self.value().to_string()
    }
}
impl<V: ToString + Clone, N: Concat> Concat for Node<V, N> {
    fn concat(&self) -> String {
        format!("{}{}", self.value().to_string(), self.next().concat())
    }
}

let list = list![1u8, "_hello", -3i32];
assert_eq!(list.forward().concat(), "1_hello-3");
assert_eq!(list.backward().concat(), "-3_hello1");
```

Example 3: We want to count how many even numbers are on the list:

```rust
use dynamic_list::*;

// Polymorphic trait
trait Even {
    fn even(&self) -> usize;
}
impl<T: Clone + TryInto<usize>> Even for T {
    fn even(&self) -> usize {
        (self.clone().try_into().unwrap_or(1) + 1) % 2
    }
}

// Iterator trait
trait NumberEven {
    fn evens(&self) -> usize;
}
impl<V: Even> NumberEven for Node<V> {
    fn evens(&self) -> usize {
        self.value().even()
    }
}
impl<V: Even + Clone, N: NumberEven> NumberEven for Node<V, N> {
    fn evens(&self) -> usize {
        self.value().even() + self.next().evens()
    }
}

let list = list![false, 1, 2u8, -3, 4isize];
assert_eq!(list.forward().evens(), 3);
```

## Limitations ⚠️

While Dynamic List provides a powerful and flexible solution, it's essential to be aware of the following limitations:

- **Trait Implementation Requirement:** To leverage the full functionality of the list, it's necessary for the values in the list to implement the required traits. Attempting to call trait methods on types that don't implement them will result in compilation errors that are hard to understand. However, with the future introduction of trait specialization in Rust, this limitation may be mitigated, allowing for more versatile trait implementations. For example, it would be possible to define a default value all types excluding the ones you are interested.
- **Heap Allocation:** Currently, elements in the list are allocated on the heap. Although this allows the possibility of avoiding the need to clone values in the list. If heap allocation is a concern, an alternative implementation allowing for single-directional allocation could be considered.

## License 📄

This project is licensed under the [MIT License](MIT-LICENSE) or [Apache License, Version 2.0](APACHE-LICENSE) at your option.
