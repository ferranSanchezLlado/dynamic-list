# Dynamic List 

A powerful and efficient implementation of dynamic lists with versatile data structures. It is designed to store any type without incurring extra costs, making it an ideal choice for a wide range of applications. One of the main advantages is the you would avoid the extra cost of dynamic dispatch.

Currently, two implementations are provided:
- *Double-linked list*: A unidirectional linked list capable of efficient iteration in both directions.
- *Array*: A fixed-size array where all elements are stored consecutively in the stack.

## Installation 🚀

Add the following to your `Cargo.toml`:

```toml
[dependencies]
dynamic-list = "0.3.0"
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

// Chain access:
let array = array![1u8, "hello", true, "world"];
assert_eq!(array.forward().next().value(), &"hello");
assert_eq!(array.backward().value(), &"world");

// Index access:
let list = list![1u8, "hello", true, "world"];
assert_eq!(Index::<U1>::index(list.forward()), &"hello");
assert_eq!(Index::<U3>::index(list.forward()), &"world");
```

Example 2: We want to concatenate a list of items into a single string:

```rust
use dynamic_list::{list::Node, *};

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
use dynamic_list::{list::Node, *};

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

- **Trait Implementation Requirement:** To leverage the full functionality of the list, it's necessary for the values in the list to implement the required traits. Attempting to call trait methods on types that don't implement them will result in compilation errors that might be challenging to decipher. However, with the anticipated introduction of trait specialization in Rust, this limitation may be mitigated, allowing for more versatile trait implementations. For instance, the ability to define a default value for all types except the ones specifically of interest could become possible. It's worth noting that trait implementation in arrays is currently more complex to achieve.
- **Heap Allocation:** Currently, elements in the list are allocated on the heap. While this approach enables avoiding the need to clone values in the list, it introduces potential concerns related to heap allocation. If heap allocation is a critical consideration for your use case, you may want to explore alternative implementations that allow for single-directional allocation, potentially reducing the impact on memory management. It's important to note that this limitation doesn't apply to the array-based implementation, which stores elements consecutively in a byte array  (which can be stored in the stack).

## License 📄

This project is licensed under the [MIT License](MIT-LICENSE) or [Apache License, Version 2.0](APACHE-LICENSE) at your option.
