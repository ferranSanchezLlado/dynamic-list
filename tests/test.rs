use dynamic_list::prelude::*;

// Iterator:
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

#[test]
fn example_one() {
    let list = DynamicList::new().push(1u8).push("_hello").push(-3i32);
    assert_eq!(list.forward().concat(), "1_hello-3");
}

// Polymorphic trait:
trait Even {
    fn even(&self) -> usize;
}
impl<T: Clone + Into<usize>> Even for T {
    fn even(&self) -> usize {
        (self.clone().into() + 1) % 2
    }
}

// Iterator:
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

#[test]
fn example_two() {
    let list = DynamicList::new().push(1u8).push(2u16).push(3usize);
    assert_eq!(list.forward().evens(), 1);
}
