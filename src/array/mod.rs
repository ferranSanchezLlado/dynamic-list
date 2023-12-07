use crate::{Empty, NotEmpty};
use std::marker::PhantomData;
use std::mem::size_of;
use traits::*;

pub(crate) mod traits;

pub const fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

// ZST
pub struct Node<V, N = Empty> {
    value: PhantomData<V>,
    next: PhantomData<N>,
}

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
    pub fn push<V>(mut self, value: V) -> Array<N, F::Output<V>, Node<V, B>> {
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

    pub const fn forward(&self) -> RefIterator<'_, F, Empty, Self> {
        RefIterator::new_forward(self)
    }

    pub const fn backward(&self) -> RefIterator<'_, B::Element, B::Rest, Self> {
        RefIterator::new_backward(self)
    }
}

pub struct RefIterator<'a, CF, CB, A> {
    array: &'a A,
    current_foreward: PhantomData<CF>,
    current_backward: PhantomData<CB>,
}

impl RefIterator<'static, Empty, Empty, Empty> {
    const fn new_forward<const N: usize, F, B>(
        array: &Array<N, F, B>,
    ) -> RefIterator<'_, F, Empty, Array<N, F, B>> {
        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }

    const fn new_backward<const N: usize, F, B: RemoveFirst>(
        array: &Array<N, F, B>,
    ) -> RefIterator<'_, B::Element, B::Rest, Array<N, F, B>> {
        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }
}

impl<'a, A, CFV, CFN: NotEmpty, CB> RefIterator<'a, Node<CFV, CFN>, CB, A> {
    pub const fn next(self) -> RefIterator<'a, CFN, Node<CFV, CB>, A> {
        let RefIterator { array, .. } = self;

        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }
}

impl<'a, A, CF, CBV, CBN> RefIterator<'a, CF, Node<CBV, CBN>, A> {
    pub const fn prev(self) -> RefIterator<'a, Node<CBV, CF>, CBN, A> {
        let RefIterator { array, .. } = self;

        RefIterator {
            array,
            current_foreward: PhantomData,
            current_backward: PhantomData,
        }
    }
}

impl<'a, const N: usize, F, B: RemoveFirst, CFV, CFN, CB: MemorySize>
    RefIterator<'a, Node<CFV, CFN>, CB, Array<N, F, B>>
{
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

    pub const fn forward(self) -> RefIterator<'a, F, Empty, Array<N, F, B>> {
        RefIterator::new_forward(self.array)
    }

    pub const fn backward(self) -> RefIterator<'a, B::Element, B::Rest, Array<N, F, B>> {
        RefIterator::new_backward(self.array)
    }
}

#[macro_export]
macro_rules! array {
    ($($x:expr),+ $(,)?) => {{
        // Fix import
        const N: usize = 0 $(+ $crate::array::size_of_val(&$x))+;
        Array::new::<N>()$(.push($x))+
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let list = Array::new::<4>().push(10);

        assert_eq!(list.forward().value(), &10);
        assert_eq!(list.backward().value(), &10);

        // TODO: Test index

        // assert_eq!(list.len(), 1);
    }

    #[test]
    fn works_n() {
        let list = Array::new::<28>().push(1).push("two").push(3.0);

        assert_eq!(list.forward().next().next().value(), &3.0);
        assert_eq!(list.backward().prev().prev().value(), &1);

        // TODO: Test index

        // assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_macro() {
        let list = array![1, "two", 3.0, true];

        let test = list.forward();
        assert_eq!(0, test.index());
        assert_eq!(&1, test.value());
    }
}
