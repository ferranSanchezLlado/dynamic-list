use crate::Empty;

pub trait NotEmpty {}

pub trait Length {
    const SIZE: usize = 0;

    fn len(&self) -> usize {
        Self::SIZE
    }

    fn is_empty(&self) -> bool {
        Self::SIZE == 0
    }
}
impl Length for Empty {}

pub trait Index<I> {
    type Output<'a>
    where
        Self: 'a;

    fn index(&self) -> Self::Output<'_>;
}
