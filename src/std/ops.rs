use core::ops::DerefMut;

pub trait DerefMove : DerefMut {
    fn deref_move(self) -> Self::Target;
}
