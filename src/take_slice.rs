/// The `TakeSlice` trait for Rust enables treating slices and `Vec` instances interchangeably.
/// 
/// # Examples
/// 
/// `take_slice` can take in both slices and vectors.
/// If it takes in a slice, no copying is needed unless `take` is called.
/// ```
/// use take_ref::TakeSlice;
/// 
/// fn take_slice(value: impl TakeSlice<i64>) {
///     assert_eq!(value.as_slice(), [1, 2, 3]); // `as_slice` references the value in place.
///     assert_eq!(value.as_slice(), [1, 2, 3]); // `as_slice` can be repeated until `take`.
///     assert_eq!(value.take(), [1, 2, 3]); // `take` consumes the value.
/// }
/// 
/// let numbers = vec!(1, 2, 3);
/// take_slice(numbers.as_slice());
/// take_slice(numbers);
/// ```
/// 
/// ## Disallowed Operations
/// 
/// `slice_taken` fails to compile since it attempts to slice `value` after `take` has already consumed it.
/// ```compile_fail
/// fn slice_taken(value: impl take_ref::TakeSlice<i64>) {
///     value.take(); // `take` consumes the value.
///     value.slice(); // This call is disallowed since the value has been consumed.
/// }
/// ```
/// 
/// `take_taken` fails to compile since it attempts to `take` `value` after `take` has already consumed it.
/// ```compile_fail
/// fn take_taken(value: impl take_ref::TakeSlice<i64>) {
///     value.take(); // `take` consumes the value.
///     value.take(); // This call is disallowed since the value has been consumed.
/// }
/// ```
pub trait TakeSlice<T: Clone> {
    /// Reference the value as a slice.
    fn as_slice(&self) -> &[T];
    /// Take ownership of the value or construct the desired value and drop `self`.
    fn take(self) -> Vec<T>;
}

impl<'a, T: Clone> TakeSlice<T> for &'a [T] {
    /// Reference the value as a slice.
    fn as_slice(&self) -> &[T] {
        *self
    }

    /// Construct the desired value and drop `self`.
    fn take(self) -> Vec<T> {
        (*self).into()
    }
}

impl<T: Clone> TakeSlice<T> for Vec<T> {
    /// Reference the value as a slice.
    fn as_slice<'a>(&self) -> &[T] {
        Vec::<T>::as_slice(self)
    }

    /// Construct the desired value and drop `self`.
    fn take(self) -> Vec<T> {
        self
    }
}
