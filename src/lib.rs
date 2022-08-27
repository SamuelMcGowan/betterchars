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
    peeked: Option<char>,
}

impl<'a> BetterChars<'a> {
    /// Create a new `StringIter` from a string.
    #[inline]
    pub fn new(s: &'a str) -> Self {
        let mut chars = s.chars();
        Self {
            s,
            peeked: chars.next(),
            chars,
        }
    }

    /// Get the current position in the string, in bytes.
    #[inline]
    pub fn pos(&self) -> usize {
        let pos = self.s.len() - self.chars.as_str().len();
        match self.peeked {
            Some(_) => pos - 1,
            None => pos,
        }
    }

    /// Get the whole underlying string.
    #[inline]
    pub fn all(&self) -> &'a str {
        self.s
    }

    /// Get the remaining part of the underlying string.
    #[inline]
    pub fn remainder(&self) -> &'a str {
        // can't `use self.chars.as_str()` because of the peeked character.
        &self.s[self.pos()..]
    }

    /// Peek at the next character without consuming it.
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.peeked
    }

    /// Consume a character if it matches.
    #[inline]
    pub fn eat(&mut self, c: char) -> Option<char> {
        if self.peek()? == c {
            return self.next();
        }
        None
    }
}

impl<'a> Iterator for BetterChars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.peeked;
        self.peeked = self.chars.next();
        res
    }
}
