use std::hash::Hash;
use std::{rc, sync};

/// Interface for elements that can be stored in a weak hash table.
pub trait WeakElement {
    /// The type at which a weak element can be viewed.
    ///
    /// For example, for `std::rc::Weak<T>`, this will be `std::rc::Rc<T>`.
    type Strong;

    /// Constructs a new weak element from a strong view.
    fn new(view: &Self::Strong) -> Self;

    /// Acquires a strong version of the weak element.
    fn view(&self) -> Option<Self::Strong>;

    /// Is the given weak element expired?
    fn expired(&self) -> bool {
        self.view().is_none()
    }
}

/// Interface for elements that can act as keys in weak hash tables.
pub trait WeakKey : WeakElement {
    /// The underlying key type.
    ///
    /// For example, for `std::rc::Weak<T>`, this will be `T`.
    type Key: Eq + Hash;

    /// Borrows a view of the key.
    fn with_key<F, R>(view: &Self::Strong, f: F) -> R
        where F: FnOnce(&Self::Key) -> R;
}

impl<T> WeakElement for rc::Weak<T> {
    type Strong = rc::Rc<T>;

    fn new(view: &Self::Strong) -> Self {
        rc::Rc::<T>::downgrade(view)
    }

    fn view(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}

impl<T: Eq + Hash> WeakKey for rc::Weak<T> {
    type Key = T;

    fn with_key<F, R>(view: &Self::Strong, f: F) -> R
        where F: FnOnce(&Self::Key) -> R
    {
        f(&view)
    }
}

impl<T> WeakElement for sync::Weak<T> {
    type Strong = sync::Arc<T>;

    fn new(view: &Self::Strong) -> Self {
        sync::Arc::<T>::downgrade(view)
    }

    fn view(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}

impl<T: Eq + Hash> WeakKey for sync::Weak<T>
{
    type Key = T;

    fn with_key<F, R>(view: &Self::Strong, f: F) -> R
        where F: FnOnce(&Self::Key) -> R
    {
        f(&view)
    }
}

