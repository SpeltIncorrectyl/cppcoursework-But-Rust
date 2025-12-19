use std::rc::Rc;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct RcKey<T>(usize, PhantomData<T>);

impl<T> RcKey<T> {
    pub fn from(rc: &Rc<T>) -> Self {
        RcKey(Rc::as_ptr(rc) as usize, PhantomData)
    }
}

impl<T> PartialEq for RcKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for RcKey<T> {}

impl<T> Hash for RcKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}