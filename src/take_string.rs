use std::string::String;

/// The `TakeString` trait for Rust enables treating `&str` and `String` instances interchangeably.
/// 
/// # Examples
/// 
/// `take_string` can take in instances of both `String` and `&str`.
/// If it takes in a `&str`, no copying is needed unless `take` is called.
/// ```
/// use take_ref::TakeString;
/// 
/// fn take_string(value: impl TakeString) {
///     assert_eq!(value.as_str(), "Hi!"); // `as_str` references the string in place.
///     assert_eq!(value.as_str(), "Hi!"); // `as_str` can be repeated until `take` is called.
///     assert_eq!(value.take(), "Hi!"); // `take` consumes the value.
/// }
/// 
/// take_string("Hi!");
/// 
/// let s = "Hi!";
/// take_string(s);
/// ```
/// 
/// ## `Disallowed Operations`
/// 
/// `ref_taken` fails to compile since it attempts to reference `value` after `take` has already consumed it.
/// ```compile_fail
/// fn ref_taken(value: impl TakeString) {
///     value.take(); // `take` consumes the value.
///     value.as_str(); // This call is disallowed since the value has already been consumed.
/// }
/// ```
/// 
/// `take_taken` fails to compile since it attempts to `take` `value` after `take` has already consumed it.
/// ```compile_fail
/// fn take_taken(value: impl TakeString) {
///     value.take(); // `take` consumes the value.
///     value.take(); // This call is disallowed since the value has already been consumed.
/// }
/// ```
pub trait TakeString {
    /// Reference the string as a `&str`.
    fn as_str(&self) -> &str;
    /// Take ownership of the String or construct a String and drop `self`.
    fn take(self) -> String;
}

impl TakeString for &str {
    /// Reference the string as a `&str`.
    fn as_str(&self) -> &str {
        self
    }

    /// Construct a String and drop `self`.
    fn take(self) -> String {
        self.into()
    }
}

impl TakeString for String {
    /// Reference the string as a `&str`.
    fn as_str(&self) -> &str {
        self
    }

    /// Take ownership of the String and drop `self`.
    fn take(self) -> String {
        self
    }
}
