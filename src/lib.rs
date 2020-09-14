//! # take_ref
//! 
//! take_ref provides the TakeRef trait.

#![crate_name = "take_ref"]

/// The TakeRef trait for Rust enables more efficient argument passing when cloning may be expensive and needed only rarely.
/// 
/// # Examples
/// 
/// ```
/// use take_ref::TakeRef;
/// 
/// // `take_value` can take in both values and references.
/// // If it takes in a reference, no clone is needed unless `take` is called.
/// fn take_value(value: impl TakeRef<i64>) {
///     assert_eq!(*value.as_ref(), 42);
///     assert_eq!(value.take(), 42);
///     // value.as_ref(); // This call fails when uncommented since the value has been consumed.
///     // value.take(); // This call fails for the same reason.
/// }
/// 
/// take_value(42);
///    
/// let i = 42;
/// take_value(&i);
///     
/// let mut i = 42;
/// take_value(&mut i);
/// ```
pub trait TakeRef<T: Clone> {
    /// Access the value by reference.
    fn as_ref(&self) -> &T;
    /// Take ownership of the value or clone the referenced value and drop `self`.
    fn take(self) -> T;
}
  
impl<T: Clone> TakeRef<T> for T {
    /// Access the value by reference.
    fn as_ref(&self) -> &T {
      self
    }
    
    /// Clone the referenced value and drop `self`.
    fn take(self) -> T {
      self
    }
}
  
impl<T: Clone> TakeRef<T> for &T {
    /// Access the reference.
    fn as_ref(&self) -> &T {
      self
    }
    
    /// Clone the referenced value and drop `self`.
    fn take(self) -> T {
      (*self).clone()
    }
}
  
impl<T: Clone> TakeRef<T> for &mut T {
    /// Access the reference.
    fn as_ref(&self) -> &T {
      self
    }
    
    /// Take ownership of the value from `self`.
    fn take(self) -> T {
      (*self).clone()
    }
}
