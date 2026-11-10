pub mod lexer;
pub mod span;

// Re-export common items for easier importing
pub use lexer::{Lexer, Token, TokenKind};
pub use span::Span;
