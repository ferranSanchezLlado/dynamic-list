# Dynamic List 

Implementation of a dynamic bidirectional linked list capable of storing any type in Rust. Therefore, through the use of this library, you can avoid the need for dynamic dispatch because the dynamic list keeps track of all the types

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
dynamic-list = "0.1.0"
```

Or if you want to use the latest version from the master branch:

```toml
[dependencies]
dynamic-list = { git = "https://github.com/ferranSanchezLlado/dynamic-list.git" }
```

## Usage

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

## Limitations

You will not be able to call the trait methods if any of the values in the list don't implement them. However, once trait specalization lands, this would result in the easy implementation of traits for all types with different behaviors. For example, the `Even` trait could make all non-numeric types default to returning 0. Therefore, the list could contain any type.

Currently, the elements in the list are being allocated in the heap. I couldn't think of any way to avoid this problem for a bidirectional list. However, I could very easily add an alternative allowing for a single direction.
## License

This project is licensed under the [MIT License](MIT-LICENSE) or [Apache License, Version 2.0](APACHE-LICENSE) at your option.
