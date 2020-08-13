use std::fmt::Debug;
use serde::Serialize;

/// An identifier in Wright source code.
/// There is only one field here, the source code of the identifier.
/// This is because the identifier itself will be the same as the
/// source.
#[derive(Clone, Debug, Serialize)]
pub struct Identifier<SourceCodeReference: Clone + Debug + Serialize> {
    /// Reference to associated source code.
    pub source: SourceCodeReference,
}

/// A scoped, or qualified, name.
#[derive(Clone, Debug, Serialize)]
pub struct ScopedName<SourceCodeReference: Clone + Debug + Serialize> {
    /// The source code fragment.
    pub source: SourceCodeReference,
    /// The sequence of simple identifiers.
    /// Example: foo::bar::baz -> [ foo, bar ]
    pub path: Vec<Identifier<SourceCodeReference>>,
    /// The final simple identifier
    /// Example: foo::bar::baz -> baz
    pub name: Identifier<SourceCodeReference>,
}

/// Numerical literal in wright source code.
/// i.e. `10`, `0xCa1a0`, `0b0101_0101`, `100_000`
#[derive(Clone, Debug, Serialize)]
pub struct NumLit<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// Represented value.
    pub inner: u128,
}

/// Character literal in wright source code.
/// i.e `'a', '\n', '\u{01f441}', '\x00', '♦'`
/// see [this page](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes) for escape
/// information.
#[derive(Clone, Debug, Serialize)]
pub struct CharLit<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// Represented Value.
    pub inner: char,
}

/// String literal in wright source code.
/// i.e. `"hello world", "with \n newline \n escapes"`
/// The parser for string literals also includes all the escape characters
/// [here](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes).
/// Raw-strings and Byte-strings (like those in rust) are not currently
/// supported but may be added in the future.
#[derive(Clone, Debug, Serialize)]
pub struct StringLit<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// Represented string value. (not a reference into source code because
    /// source code may contain escaped characters.)
    pub inner: String,
}

/// Boolean literal in wright source code.
/// i.e. `true`, `false`.
#[derive(Clone, Debug, Serialize)]
pub struct BooleanLit<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// Represented value.
    pub inner: bool,
}

/// `self` literal in wright source code.
#[derive(Clone, Debug, Serialize)]
pub struct SelfLit<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
}
