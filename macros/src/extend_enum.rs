use proc_macro2::TokenStream;

/// This module provide a macro to extend an enum with variants from another enum.
/// Latter variants take precedence over earlier ones.
/// ```
/// use macros::extend_enum;
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum A {
///     A,
///     B,
///     C,
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum B {
///     C,
///     D,
/// }
///
/// extend_enum!(All, A, B);
/// ```
///
/// The above code will generate the following code:
/// ```
/// #[derive(Debug, PartialEq, Eq)]
/// enum All {
///     A,
///     B,
///     C, // from B
///     D,
/// }
///
/// impl From<A> for All {
///    fn from(a: A) -> Self {
///         match a {
///             A::A => All::A,
///             A::B => All::B,
///        }
///   }
/// }
///
/// impl From<B> for All {
///     fn from(b: B) -> Self {
///         match b {
///             B::C => All::C,
///             B::D => All::D,
///       }
///  }
/// }    
///  
/// ```   
///
pub fn extend_enum(input: TokenStream) -> TokenStream {
    todo!()
}
