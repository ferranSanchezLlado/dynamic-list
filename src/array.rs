#![allow(unused)]
use crate::{Empty, NotEmpty};
use std::marker::PhantomData;
use std::mem::size_of;

const fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

// ZST
pub struct ArrayNode<V, N = Empty> {
    value: PhantomData<V>,
    next: PhantomData<N>,
}

pub trait ArrayAppend {
    type Output<T>;
}
impl ArrayAppend for Empty {
    type Output<T> = ArrayNode<T>;
}
impl<V, N: ArrayAppend> ArrayAppend for ArrayNode<V, N> {
    type Output<T> = ArrayNode<V, N::Output<T>>;
}

pub trait MemorySize {
    const SIZE: usize = 0;
}
impl MemorySize for Empty {}
impl<V, N: MemorySize> MemorySize for ArrayNode<V, N> {
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
impl<V, N> RemoveFirst for ArrayNode<V, N> {
    type Element = ArrayNode<V>;
    type Rest = N;
}

impl<V> NotEmpty for ArrayNode<V> {}
impl<V, N: NotEmpty> NotEmpty for ArrayNode<V, N> {}

#[repr(transparent)]
pub struct Array<const N: usize, F, B> {
    data: [u8; N], // Array of bytes
    forward: PhantomData<F>,
    backward: PhantomData<B>,
}

impl Array<0, Empty, Empty> {
    pub const fn new<const N: usize>() -> Array<N, Empty, Empty> {
        Array {
            data: [0; N],
            forward: PhantomData,
            backward: PhantomData,
        }
    }
}

impl<const N: usize, F: ArrayAppend, B: MemorySize + RemoveFirst> Array<N, F, B> {
    pub fn push<V>(mut self, value: V) -> Array<N, F::Output<V>, ArrayNode<V, B>> {
        assert!(
            size_of::<V>() + B::SIZE <= N,
            "The element doesn't fit in the array"
        );
        // TODO: ChecK N is doesn't wrap around
        // TODO: Try to cast transmute self without moving it
        unsafe { self.data.as_mut_ptr().add(B::SIZE).cast::<V>().write(value) };
        let Array { data, .. } = self;

        Array {
            data,
            forward: PhantomData,
            backward: PhantomData,
        }
    }

    pub const fn forward(&self) -> RefIterator<'_, N, F, B, F, Empty> {
        RefIterator::new_forward(self)
    }

    pub const fn backward(&self) -> RefIterator<'_, N, F, B, B::Element, B::Rest> {
        RefIterator::new_backward(self)
    }
}

pub struct RefIterator<'a, const N: usize, F, B, CF, CB> {
    array: &'a Array<N, F, B>,
    current_foreward: PhantomData<CF>,
    current_backward: PhantomData<CB>,
}

impl RefIterator<'static, 0, Empty, Empty, Empty, Empty> {
    const fn new_forward<const N: usize, F, B>(
        array: &Array<N, F, B>,
    ) -> RefIterator<'_, N, F, B, F, Empty> {
        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }

    const fn new_backward<const N: usize, F, B: RemoveFirst>(
        array: &Array<N, F, B>,
    ) -> RefIterator<'_, N, F, B, B::Element, B::Rest> {
        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }
}

impl<'a, const N: usize, F, B, CFV, CFN: NotEmpty, CB: MemorySize>
    RefIterator<'a, N, F, B, ArrayNode<CFV, CFN>, CB>
{
    const fn next(self) -> RefIterator<'a, N, F, B, CFN, ArrayNode<CFV, CB>> {
        let RefIterator { array, .. } = self;

        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }

    pub const fn index(&self) -> usize {
        CB::SIZE
    }

    pub fn value(&self) -> &CFV {
        unsafe {
            // TODO: Find a way to fix unaligned error
            self.array
                .data
                .as_ptr()
                .add(self.index())
                .cast::<CFV>()
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

impl<'a, const N: usize, F, B, CF, CBV, CBN> RefIterator<'a, N, F, B, CF, ArrayNode<CBV, CBN>> {
    const fn prev(self) -> RefIterator<'a, N, F, B, ArrayNode<CBV, CF>, CBN> {
        let RefIterator { array, .. } = self;

        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }
}

#[macro_export]
macro_rules! array {
    ($($x:expr),+ $(,)?) => {{
        const N: usize = 0 $(+ size_of_val(&$x))+;
        Array::new::<N>()$(.push($x))+
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let list = Array::new::<4>().push(u32::MAX - 1);

        // assert_eq!(list.forward.value(), &1);
        // assert_eq!(list.backward.value(), &1);
        //
        // assert_eq!(list.forward.next, Empty);
        // assert_eq!(list.backward.next, Empty);
        //
        // assert_eq!(list.len(), 1);
    }

    #[test]
    fn works_n() {
        // let list = DynamicList::new().push(1).push("two").push(3.0);
        //
        // assert_eq!(list.forward.next.next.value(), &3.0);
        // assert_eq!(list.backward.next.next.value(), &1);
        //
        // assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_macro() {
        let list_1 = array![1_i32, "two", 3.0, true];
        //let list_2 = list!().push(1).push("two").push(3.0).push(true);

        let test = list_1.forward();
        assert_eq!(0, test.index());
        assert_eq!(&1, test.value());
    }
}
