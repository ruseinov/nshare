#[cfg(feature = "ndarray")]
mod ndarray_impl;

/// Converts a 1 or 2 dimensional type to a nalgebra type.
///
/// This uses an associated type to avoid ambiguity for the compiler.
/// By calling this, the compiler always knows the returned type.
pub trait IntoNalgebra {
    type Out;

    fn into_nalgebra(self) -> Self::Out;
}
