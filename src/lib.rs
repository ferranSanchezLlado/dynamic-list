#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
struct Empty;

const E: *const Empty = &Empty as *const Empty;
struct Node<V, N = Empty> {
    value: *const V,
    next: N,
}

impl Node<Empty, Empty> {
    fn new<V>(value: *const V) -> Node<V> {
        Node { value, next: Empty }
    }
}

impl<V> Node<V> {
    // fn append<V2>(self, value: *const V2) -> Node<V, Node<V2>> {
    //     Node {
    //         value: self.value,
    //         next: Node { value, next: Empty },
    //     }
    // }
}

impl<V, N> Node<V, N> {
    fn prepend<V2>(self, value: *const V2) -> Node<V2, Self> {
        Node { value, next: self }
    }

    fn value(&self) -> V
    where
        V: Clone,
    {
        // We ensure value is clone after reconsturcting the object
        unsafe { self.value.read() }.clone()
    }
}

trait Append {
    type NewType<T>;

    fn append<T>(self, value: *const T) -> Self::NewType<T>;
}

impl<V> Append for Node<V> {
    type NewType<T> = Node<V, Node<T>>;

    fn append<T>(self, value: *const T) -> Self::NewType<T> {
        Node {
            value: self.value,
            next: Node { value, next: Empty },
        }
    }
}

impl<V, N: Append> Append for Node<V, N> {
    type NewType<T> = Node<V, N::NewType<T>>;

    fn append<T>(self, value: *const T) -> Self::NewType<T> {
        Node {
            value: self.value,
            next: self.next.append(value),
        }
    }
}

pub struct DynamicList<F, B> {
    forward: F,
    backward: B,
}

impl DynamicList<Empty, Empty> {
    fn new() -> Self {
        Self {
            forward: Empty,
            backward: Empty,
        }
    }

    fn add<V>(self, value: V) -> DynamicList<Node<V>, Node<V>> {
        let value = Box::into_raw(Box::new(value));

        DynamicList {
            forward: Node::new(value),
            backward: Node::new(value),
        }
    }
}

impl<F: Append, BV, BN> DynamicList<F, Node<BV, BN>> {
    fn add<V>(self, value: V) -> DynamicList<F::NewType<V>, Node<V, Node<BV, BN>>> {
        let value = Box::into_raw(Box::new(value));

        DynamicList {
            forward: self.forward.append(value),
            backward: self.backward.prepend(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let list = DynamicList::new().add(1);

        assert_eq!(list.forward.value(), 1);
        assert_eq!(list.backward.value(), 1);

        assert_eq!(list.forward.next, Empty);
        assert_eq!(list.backward.next, Empty);
    }

    #[test]
    fn works_n() {
        let list = DynamicList::new().add(1).add("two").add(3.0);

        assert_eq!(list.forward.next.next.value(), 3.0);
        assert_eq!(list.backward.next.next.value(), 1);
    }

    trait Truth {
        const TRUE: bool = true;
    }
    impl<T> Truth for T {}
}
