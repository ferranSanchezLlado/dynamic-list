#![allow(dead_code)]
use std::ptr::null;

struct Empty;

// const E: *const Empty = &Empty as *const Empty;
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
    fn append<V2>(self, value: *const V2) -> Node<V, Node<V2>> {
        Node {
            value: self.value,
            next: Node::new(value),
        }
    }
}

impl<V, N> Node<V, N> {
    fn prepend<V2>(self, value: *const V2) -> Node<V2, Node<V, N>> {
        Node { value, next: self }
    }

    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    fn value(&self) -> V
    where
        V: Clone,
    {
        unsafe { self.value.read() }.clone()
    }
}

// impl Drop

struct TypedLinkList<S, E> {
    start: S, // Single linked list: left-right
    end: E,   // Single linked list: right-left

    // Pointer to right-most element in linked list
    _start_end: *const (),
}

impl TypedLinkList<Empty, Empty> {
    pub fn new() -> Self {
        Self {
            start: Empty,
            end: Empty,
            _start_end: null(),
        }
    }

    // Appends right-most position
    pub fn add<V>(self, value: V) -> TypedLinkList<Node<V>, Node<V>> {
        let value = Box::into_raw(Box::new(value));

        TypedLinkList {
            start: Node::new(value),
            end: Node::new(value),
            _start_end: null(),
        }
    }
}

// Case 1 element
impl<V> TypedLinkList<Node<V>, Node<V>> {
    pub fn add<V2>(self, value: V2) -> TypedLinkList<Node<V, Node<V2>>, Node<V2, Node<V>>> {
        let value = Box::into_raw(Box::new(value));

        let start = self.start.append(value);
        let start_end = start.next.as_ptr().cast();

        TypedLinkList {
            start,
            end: self.end.prepend(value),
            _start_end: start_end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_2() {
        let a = TypedLinkList::new().add(10).add("hi");

        assert_eq!(a.start.value(), 10);
        assert_eq!(a.start.next.value(), "hi");
    }
}
