/// The `TakeRef` trait for Rust enables treating references and values interchangeably.
/// 
/// # Examples
/// 
/// `take_value` can take in both values and references.
/// If it takes in a reference, no clone is needed unless `take` is called.
/// ```
/// use take_ref::TakeRef;
/// 
/// fn take_value(value: impl TakeRef<i64>) {
///     assert_eq!(*value.as_ref(), 42); // `as_ref` references the value in place.
///     assert_eq!(*value.as_ref(), 42); // `as_ref` can be repeated until `take` is called.
///     assert_eq!(value.take(), 42); // `take` consumes the value.
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
/// 
/// ## Disallowed Operations
/// 
/// `ref_taken` fails to compile since it attempts to reference `value` after `take` has already consumed it.
/// ```compile_fail
/// fn ref_taken(value: impl take_ref::TakeRef<i64>) {
///     value.take(); // `take` consumes the value.
///     value.as_ref(); // This call is disallowed since the value has been consumed.
/// }
/// ```
/// 
/// `take_taken` fails to compile since it attempts to `take` `value` after `take` has already consumed it.
/// ```compile_fail
/// fn take_taken(value: impl take_ref::TakeRef<i64>) {
///     value.take(); // `take` consumes the value.
///     value.take(); // This call is disallowed since the value has been consumed.
/// }
/// ```

pub trait TakeRef<T: Clone> {
    /// Access the value by reference.
    fn as_ref(&self) -> &T;
    /// Take ownership of the value or clone the referenced value and drop `self`.
    fn take(self) -> T;
}

impl<T: Clone> TakeRef<T> for &T {
    /// Access the reference.
    fn as_ref(&self) -> &T {
        self
    }
    
    /// Clone the referenced value and drop `self`.
    fn take(self) -> T {
        self.clone()
    }
}
  
impl<T: Clone> TakeRef<T> for &mut T {
    /// Access the reference.
    fn as_ref(&self) -> &T {
        self
    }
    
    /// Clone the referenced value and drop `self`.
    fn take(self) -> T {
        self.clone()
    }
}

impl<T: Clone> TakeRef<T> for T {
    /// Access the value by reference.
    fn as_ref(&self) -> &T {
        self
    }
    
    /// Take ownership of the value and drop `self`.
    fn take(self) -> T {
        self
    }
}
