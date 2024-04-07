use std::{
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
};

pub struct Ref<T: ?Sized> {
    ptr: Box<T>,
}

#[allow(non_snake_case)]
pub fn Ref<T: 'static>(val: T) -> Ref<T> {
    Ref { ptr: Box::new(val) }
}

impl<T: 'static> Ref<T> {
    pub fn then<F, R>(self, f: F) -> R
    where
        F: FnOnce(T) -> R,
    {
        f(*self.ptr)
    }

    pub fn get_inner(self) -> T {
        *self.ptr
    }

    pub fn map_into<F>(mut self, f: F) -> Ref<T>
    where
        F: FnOnce(T) -> T,
    {
        let transformed = f(*self.ptr);
        *self.ptr = transformed;
        self
    }
}

impl<T: ?Sized> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ptr
    }
}

impl<T: ?Sized> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ptr
    }
}

impl<T: 'static + Clone> Clone for Ref<T> {
    fn clone(&self) -> Ref<T> {
        Ref((**self).clone())
    }
}

impl<T: ?Sized + Debug> Debug for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.ptr, f)
    }
}

impl<T: Display> Display for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<T> fmt::Pointer for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let ptr = Ref(10);
        assert_eq!(*ptr, 11);
        assert_eq!(ptr.clone().get_inner(), 10);
    }
}
