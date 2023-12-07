use std::mem::size_of;

use crate::array::Node;
use crate::{Empty, NotEmpty};

pub trait ArrayAppend {
    type Output<T>;
}
impl ArrayAppend for Empty {
    type Output<T> = Node<T>;
}
impl<V, N: ArrayAppend> ArrayAppend for Node<V, N> {
    type Output<T> = Node<V, N::Output<T>>;
}

pub trait MemorySize {
    const SIZE: usize = 0;
}
impl MemorySize for Empty {}
impl<V, N: MemorySize> MemorySize for Node<V, N> {
    const SIZE: usize = size_of::<V>() + N::SIZE;
}

pub trait RemoveFirst {
    type Element;
    type Rest;
}
impl RemoveFirst for Empty {
    type Element = Empty;
    type Rest = Empty;
}
impl<V, N> RemoveFirst for Node<V, N> {
    type Element = Node<V>;
    type Rest = N;
}

impl<V> NotEmpty for Node<V> {}
impl<V, N: NotEmpty> NotEmpty for Node<V, N> {}
