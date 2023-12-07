use crate::Empty;

pub trait NotEmpty {}

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
