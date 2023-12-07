#[test]
fn example_one() {
    use dynamic_list::*;
    use typenum::*;

    let list = list![1u8, "hello", true, "world"];
    assert_eq!(Index::<U1>::index(list.forward()), &"hello");
    assert_eq!(Index::<U3>::index(list.forward()), &"world");
}

#[test]
fn example_two() {
    use dynamic_list::{list::Node, *};
    // Iterator
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
}

#[test]
fn example_three() {
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

    // Iterator
    trait NumberEven {
        fn evens(&self) -> usize;
    }
    impl<V: Even> NumberEven for Node<V> {
        fn evens(&self) -> usize {
            self.value().even()
        }
    }
    impl<V: Even, N: NumberEven> NumberEven for Node<V, N> {
        fn evens(&self) -> usize {
            self.value().even() + self.next().evens()
        }
    }

    let list = list![false, 1, 2u8, -3, 4isize];
    assert_eq!(list.forward().evens(), 3);
}
