use core::ops::Deref;

pub trait DerefMove : Deref {
    fn deref_move(self) -> Self::Target;
}
