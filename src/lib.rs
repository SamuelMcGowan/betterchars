#![no_std]

use core::str::Chars;

pub trait IntoBetterChars {
    fn better_chars(&self) -> BetterChars;
}

impl<S: AsRef<str>> IntoBetterChars for S {
    /// Create new a `BetterChars` iterator.
    ///
    /// See `BetterChars` for more information.
    #[inline]
    fn better_chars(&self) -> BetterChars {
        BetterChars::new(self.as_ref())
    }
}

impl<'a> From<&'a str> for BetterChars<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        BetterChars::new(s)
    }
}

/// An iterator over the characters of a string.
///
/// Allows peeking and provides a method to get the current
/// position in the string.
#[derive(Debug, Clone)]
pub struct BetterChars<'a> {
    s: &'a str,
    chars: Chars<'a>,
}

impl<'a> BetterChars<'a> {
    /// Create a new `StringIter` from a string.
    #[inline]
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            chars: s.chars(),
        }
    }

    /// Get the current position in the string, in bytes.
    #[inline]
    pub fn pos(&self) -> usize {
        self.s.len() - self.chars.as_str().len()
    }

    /// Get the whole underlying string.
    #[inline]
    pub fn all(&self) -> &'a str {
        self.s
    }

    /// Get the remaining part of the underlying string.
    #[inline]
    pub fn remainder(&self) -> &'a str {
        self.chars.as_str()
    }

    /// Peek at the next character without consuming it.
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Consume a character if it matches.
    #[inline]
    pub fn eat(&mut self, c: char) -> Option<char> {
        if self.peek()? == c {
            return self.next();
        }
        None
    }

    /// Consume a string if it matches.
    #[inline]
    pub fn eat_str(&mut self, s: &str) -> Option<&'a str> {
        if self.remainder().starts_with(s) {
            let (prefix, remainder) = self.remainder().split_at(s.len());
            self.chars = remainder.chars();
            return Some(prefix);
        }

        None
    }
}

impl<'a> Iterator for BetterChars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_str() {
        let mut s = "foobar".better_chars();

        assert_eq!(s.eat_str("foo"), Some("foo"));
        assert_eq!(s.remainder(), "bar");

        assert_eq!(s.eat_str("bar"), Some("bar"));
        assert_eq!(s.remainder(), "");
    }

    #[test]
    fn eat_nothing() {
        let mut s = "hello".better_chars();

        assert_eq!(s.eat_str("hi"), None);
        assert_eq!(s.remainder(), "hello");
    }
}
