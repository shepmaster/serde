//! A stand-in for `std::error`
use core::any::TypeId;
use core::fmt::{Debug, Display};

/// A stand-in for `std::error::Error`, which requires no allocation.
pub trait Error: Debug + Display {
    /// A short description of the error.
    ///
    /// The description should not contain newlines or sentence-ending
    /// punctuation, to facilitate embedding in larger user-facing
    /// strings.
    fn description(&self) -> &str;

    /// The lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> { None }

    /// Get the `TypeId` of `self`
    #[doc(hidden)]
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}
