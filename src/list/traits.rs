use crate::{list::Node, Empty, Index, Length, NotEmpty};
use std::ops::Sub;
use typenum::{Bit, NonZero, Sub1, UInt, Unsigned, B1, U0};

/// # Safety
/// You should not need to ever use it directly.
///
/// If you call this method manually this could lead to a double free error.
pub unsafe trait DropValue {
    /// # Safety
    /// This method recursively drops all the values from the heap. Therfore multiple calls to this
    /// method could lead to errors.
    unsafe fn drop_values(&mut self) {}
}
unsafe impl DropValue for Empty {}
unsafe impl<V, N: DropValue> DropValue for Node<V, N> {
    unsafe fn drop_values(&mut self) {
        drop(Box::from_raw(self.value.cast_mut()));
        self.next.drop_values()
    }
}

pub trait ListAppend {
    type Output<T>: DropValue;

    fn append<T>(self, value: *const T) -> Self::Output<T>;
}
impl<V> ListAppend for Node<V> {
    type Output<T> = Node<V, Node<T>>;

    #[inline]
    fn append<T>(self, value: *const T) -> Self::Output<T> {
        Node {
            value: self.value,
            next: Node::new(value),
        }
    }
}
impl<V, N: ListAppend + DropValue> ListAppend for Node<V, N> {
    type Output<T> = Node<V, N::Output<T>>;

    #[inline]
    fn append<T>(self, value: *const T) -> Self::Output<T> {
        Node {
            value: self.value,
            next: self.next.append(value),
        }
    }
}

// INDEX:
// Case: Fount element
impl<V, N> Index<U0> for Node<V, N> {
    type Output<'a> = &'a V where Self: 'a;

    fn index(&self) -> Self::Output<'_> {
        self.value()
    }
}
// Case: There still index remaining but we arrived to last element
impl<V, U: Unsigned, B: Bit> Index<UInt<U, B>> for Node<V>
where
    UInt<U, B>: NonZero,
{
    type Output<'a> = Empty where Self: 'a;

    fn index(&self) -> Self::Output<'_> {
        Empty
    }
}
// Case: Generic search recursive search for element when not in last node
impl<V, U, B, N> Index<UInt<U, B>> for Node<V, N>
where
    U: Unsigned,
    B: Bit,
    N: NotEmpty + Index<Sub1<UInt<U, B>>>,
    UInt<U, B>: NonZero + Sub<B1>,
{
    type Output<'a> = N::Output<'a> where Self: 'a;

    fn index(&self) -> Self::Output<'_> {
        self.next.index()
    }
}

impl<V, N: Length> Length for Node<V, N> {
    const SIZE: usize = 1 + N::SIZE;
}

impl<V> NotEmpty for Node<V> {}
impl<V, N: NotEmpty> NotEmpty for Node<V, N> {}
