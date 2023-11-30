use std::mem::forget;
use std::ops::Sub;
use typenum::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Empty;

pub struct Node<V, N = Empty> {
    value: *const V,
    next: N,
}
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

impl Node<Empty, Empty> {
    #[inline]
    const fn new<V>(value: *const V) -> Node<V> {
        Node { value, next: Empty }
    }
}

impl<V, N> Node<V, N> {
    #[inline]
    const fn prepend<V2>(self, value: *const V2) -> Node<V2, Self> {
        Node { value, next: self }
    }
    #[inline]
    pub const fn value(&self) -> &V {
        // Safety: Value read should never be null, as the value is being stored in the heap.
        // See DynamicList add method.
        unsafe { &*self.value }
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut V {
        // Safety: Value read should never be null, as the value is being stored in the heap.
        // See DynamicList add method.
        unsafe { self.value.cast_mut().as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub const fn next(&self) -> &N {
        &self.next
    }

    #[inline]
    pub fn next_mut(&mut self) -> &mut N {
        &mut self.next
    }
}

// TODO: Test splitting append method from type system
pub trait Append {
    type Output<T>: DropValue;

    fn append<T>(self, value: *const T) -> Self::Output<T>;
}

impl<V> Append for Node<V> {
    type Output<T> = Node<V, Node<T>>;

    #[inline]
    fn append<T>(self, value: *const T) -> Self::Output<T> {
        Node {
            value: self.value,
            next: Node::new(value),
        }
    }
}

impl<V, N: Append + DropValue> Append for Node<V, N> {
    type Output<T> = Node<V, N::Output<T>>;

    #[inline]
    fn append<T>(self, value: *const T) -> Self::Output<T> {
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

pub trait NotEmpty {}
impl<V> NotEmpty for Node<V> {}
impl<V, N: NotEmpty> NotEmpty for Node<V, N> {}

pub trait Index<I> {
    type Output<'a>
    where
        Self: 'a;

    fn index(&self) -> Self::Output<'_>;
}
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

pub struct DynamicList<F, B: DropValue> {
    forward: F,
    backward: B,
}

impl DynamicList<Empty, Empty> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            forward: Empty,
            backward: Empty,
        }
    }

    #[inline]
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

impl<F, N: Size + DropValue> DynamicList<F, N> {
    #[inline]
    pub const fn forward(&self) -> &F {
        &self.forward
    }

    #[inline]
    pub fn forward_mut(&mut self) -> &mut F {
        &mut self.forward
    }

    #[inline]
    pub const fn backward(&self) -> &N {
        &self.backward
    }

    #[inline]
    pub fn backward_mut(&mut self) -> &mut N {
        &mut self.backward
    }

    #[inline]
    pub const fn len(&self) -> usize {
        N::SIZE
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        N::SIZE == 0
    }
}

impl<F: Append, BV, BN: DropValue> DynamicList<F, Node<BV, BN>> {
    #[inline]
    pub fn push<V>(self, value: V) -> DynamicList<F::Output<V>, Node<V, Node<BV, BN>>>
    where
        <F as Append>::Output<V>: DropValue,
    {
        let value = Box::into_raw(Box::new(value));

        // Safety: We are deconstructing the struct while avoiding calling the drop method. As this
        // method should only be called at the end by the user.
        let forward = unsafe { (&self.forward as *const F).read() };
        let backward = unsafe { (&self.backward as *const Node<BV, BN>).read() };
        forget(self);

        DynamicList {
            forward: forward.append(value),
            backward: backward.prepend(value),
        }
    }
}

impl<F, N: DropValue> Drop for DynamicList<F, N> {
    fn drop(&mut self) {
        // Safety: We can should only call the recursive drop method in one of the branches:
        // Otherwise this could lead to double free calls.
        unsafe {
            self.backward.drop_values();
        }
    }
}

pub mod prelude {
    pub use crate::{list, DynamicList, Node, Size};
}

#[macro_export]
macro_rules! list {
    () => {
        DynamicList::new()
    };
    ($($x:expr),+ $(,)?) => {
        DynamicList::new()$(.push($x))+
    };
}

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

    #[test]
    fn test_macro() {
        let list_1 = list![1, "two", 3.0, true];
        let list_2 = list!().push(1).push("two").push(3.0).push(true);

        assert_eq!(list_1.len(), 4);
        assert_eq!(list_2.len(), 4);
    }

    #[test]
    fn test_drop() {
        static mut N_DROPS: u8 = 0;

        fn drops() -> u8 {
            unsafe { N_DROPS }
        }

        struct Test;

        impl Drop for Test {
            fn drop(&mut self) {
                unsafe {
                    N_DROPS += 1;
                }
            }
        }

        // Test that the struct Test implemented properly drop
        drop(Test);
        assert_eq!(drops(), 1);

        let list = list![Test, Test, Test];
        drop(list);
        assert_eq!(drops(), 4);
    }

    #[test]
    fn test_index() {
        let list_1 = list![1, "two", 3.0, true];

        assert_eq!(&1, Index::<U0>::index(list_1.forward()));
        assert_eq!(&"two", Index::<U1>::index(list_1.forward()));
        assert_eq!(&3.0, Index::<U2>::index(list_1.forward()));
        assert_eq!(&true, Index::<U3>::index(list_1.forward()));
        assert_eq!(Empty, Index::<U4>::index(list_1.forward()));

        assert_eq!(Empty, Index::<U100>::index(list_1.forward()));
    }
}
