#[derive(Debug, PartialEq, Eq)]
pub struct Empty;

pub struct Node<V, N = Empty> {
    value: *const V,
    next: N,
}

impl Node<Empty, Empty> {
    fn new<V>(value: *const V) -> Node<V> {
        Node { value, next: Empty }
    }
}

impl<V, N> Node<V, N> {
    fn prepend<V2>(self, value: *const V2) -> Node<V2, Self> {
        Node { value, next: self }
    }

    pub fn value(&self) -> &V {
        // SAFETY: Value read should never be null, as the the pointer is allocated in value is from
        // the heap. See DynamicList add method.
        unsafe { self.value.as_ref().unwrap() }
    }

    pub fn value_mut(&mut self) -> &mut V {
        // SAFETY: Value read should never be null, as the the pointer is allocated in value is from
        // the heap. See DynamicList add method.
        unsafe { self.value.cast_mut().as_mut().unwrap() }
    }

    pub fn next(&self) -> &N {
        &self.next
    }

    pub fn next_mut(&mut self) -> &mut N {
        &mut self.next
    }
}

pub trait Append {
    type NewType<T>;

    fn append<T>(self, value: *const T) -> Self::NewType<T>;
}

impl<V> Append for Node<V> {
    type NewType<T> = Node<V, Node<T>>;

    fn append<T>(self, value: *const T) -> Self::NewType<T> {
        Node {
            value: self.value,
            next: Node::new(value),
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

pub trait Size {
    const SIZE: usize = 0;

    fn len(&self) -> usize {
        Self::SIZE
    }

    fn is_empty(&self) -> bool {
        Self::SIZE == 0
    }
}
impl Size for Empty {}
impl<V, N: Size> Size for Node<V, N> {
    const SIZE: usize = 1 + N::SIZE;
}

pub struct DynamicList<F, B> {
    forward: F,
    backward: B,
}

impl DynamicList<Empty, Empty> {
    pub const fn new() -> Self {
        Self {
            forward: Empty,
            backward: Empty,
        }
    }

    pub fn push<V>(self, value: V) -> DynamicList<Node<V>, Node<V>> {
        let value = Box::into_raw(Box::new(value));

        DynamicList {
            forward: Node::new(value),
            backward: Node::new(value),
        }
    }
}

impl Default for DynamicList<Empty, Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Size, N> DynamicList<F, N> {
    pub fn forward(&self) -> &F {
        &self.forward
    }

    pub fn forward_mut(&mut self) -> &mut F {
        &mut self.forward
    }

    pub fn backward(&self) -> &N {
        &self.backward
    }

    pub fn backward_mut(&mut self) -> &mut N {
        &mut self.backward
    }

    pub const fn len(&self) -> usize {
        F::SIZE
    }

    pub const fn is_empty(&self) -> bool {
        F::SIZE == 0
    }
}

impl<F: Append, BV, BN> DynamicList<F, Node<BV, BN>> {
    pub fn push<V>(self, value: V) -> DynamicList<F::NewType<V>, Node<V, Node<BV, BN>>> {
        let value = Box::into_raw(Box::new(value));

        DynamicList {
            forward: self.forward.append(value),
            backward: self.backward.prepend(value),
        }
    }
}

pub mod prelude {
    pub use crate::{DynamicList, Node, Size};
}

// TODO: Handle the dropping of a dynamic list

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let list = DynamicList::new().push(1);

        assert_eq!(list.forward.value(), &1);
        assert_eq!(list.backward.value(), &1);

        assert_eq!(list.forward.next, Empty);
        assert_eq!(list.backward.next, Empty);

        assert_eq!(list.len(), 1);
    }

    #[test]
    fn works_n() {
        let list = DynamicList::new().push(1).push("two").push(3.0);

        assert_eq!(list.forward.next.next.value(), &3.0);
        assert_eq!(list.backward.next.next.value(), &1);

        assert_eq!(list.len(), 3);
    }
}
