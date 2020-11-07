use std::str::CharIndices;
use std::iter::FusedIterator;
use common::{Peekable, AdvancedIter, Accept};

use log::debug;

/// Like Peekable<CharIndices> except the iterator is advanced on at the start.
#[derive(Debug, Clone)]
pub struct AdvancedChars<'a> {
    chars: AdvancedIter<CharIndices<'a>>,
    peek_ch: Option<char>,
    peek_pos: Option<usize>,
    current_pos: Option<usize>,
    current_ch: Option<char>,
    length: usize,
}

impl<'a> AdvancedChars<'a> {
    /// Creates a new advanced char iterator from a string.
    pub fn new(input: &str) -> AdvancedChars<'_> {
        let chars = AdvancedIter::new(input.char_indices());
        let (peek_pos, peek_ch) = match chars.peek_item() {
            Some((idx, ch)) => (Some(*idx), Some(*ch)),
            None => (None, None)
        };
        let current_pos = None;
        let current_ch = None;
        let length = input.len();
        AdvancedChars {
            chars,
            peek_ch,
            peek_pos,
            current_pos,
            current_ch,
            length,
        }
    }

    pub fn peek_pos(&self) -> Option<usize> {
        self.peek_pos
    }

    pub fn peek_pos_or_end(&self) -> usize {
        self.peek_pos.unwrap_or(self.length)
    }

    pub fn current_pos(&self) -> Option<usize> {
        self.current_pos
    }

    pub fn current_pos_or_end(&self) -> usize {
        self.current_pos.unwrap_or(self.length)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<'a> Peekable for AdvancedChars<'a> {
    fn peek(&self) -> Option<&char> {
        let res = self.peek_ch.as_ref();
        debug!("peeked char: {:?}", res);
        res
    }
}

impl<'a> Iterator for AdvancedChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (current_pos, current_ch) = self.chars.next().unzip();
        self.current_pos = current_pos;
        self.current_ch = current_ch;
        let (peek_pos, peek_ch) = self.chars.peek().map(|i| *i).unzip();
        self.peek_pos = peek_pos;
        self.peek_ch = peek_ch;
        current_ch
    }
}

trait OptionExt<T, U> {
    fn unzip(self) -> (Option<T>, Option<U>);
}

impl<T, U> OptionExt<T, U> for Option<(T, U)> {
    fn unzip(self) -> (Option<T>, Option<U>) {
        self.map_or((None, None), |(t, u)| {
            (Some(t), Some(u))
        })
    }
}

impl<'a> FusedIterator for AdvancedChars<'a> {  }

impl<'a> Accept<char> for AdvancedChars<'a> {  }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        // initialize
        let mut advanced_chars = AdvancedChars::new("");
        assert_eq!(advanced_chars.peek(), None);
        assert_eq!(advanced_chars.peek_pos(), None);
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.len(), 0);

        // first iteration (test for fused)
        assert_eq!(advanced_chars.next(), None);
        assert_eq!(advanced_chars.peek(), None);
        assert_eq!(advanced_chars.peek_pos(), None);
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.len(), 0);
    }

    #[test]
    fn one_char_input() {
        let mut advanced_chars = AdvancedChars::new("a");
        assert_eq!(advanced_chars.peek(), Some(&'a'));
        assert_eq!(advanced_chars.peek_pos(), Some(0));
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.len(), 1);

        assert_eq!(advanced_chars.next(), Some('a'));
        assert_eq!(advanced_chars.peek(), None);
        assert_eq!(advanced_chars.peek_pos(), None);
        assert_eq!(advanced_chars.current_pos(), Some(0));
        assert_eq!(advanced_chars.len(), 1);

        // third iteration (test for fused)
        assert_eq!(advanced_chars.next(), None);
        assert_eq!(advanced_chars.peek(), None);
        assert_eq!(advanced_chars.peek_pos(), None);
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.len(), 1);
    }

    #[test]
    fn advanced_chars_simple() {
        let mut advanced_chars = AdvancedChars::new("hi");
        // initialized test
        assert_eq!(advanced_chars.peek(), Some(&'h'));
        assert_eq!(advanced_chars.peek_pos(), Some(0));
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.len(), 2);

        // first iteration
        assert_eq!(advanced_chars.next(), Some('h'));
        assert_eq!(advanced_chars.peek(), Some(&'i'));
        assert_eq!(advanced_chars.current_pos(), Some(0));
        assert_eq!(advanced_chars.peek_pos(), Some(1));
        assert_eq!(advanced_chars.len(), 2);

        // second iteration
        assert_eq!(advanced_chars.next(), Some('i'));
        assert_eq!(advanced_chars.peek(), None);
        assert_eq!(advanced_chars.current_pos(), Some(1));
        assert_eq!(advanced_chars.peek_pos(), None);
        assert_eq!(advanced_chars.len(), 2);

        // third iteration (test for fused)
        assert_eq!(advanced_chars.next(), None);
        assert_eq!(advanced_chars.peek(), None);
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.peek_pos(), None);
        assert_eq!(advanced_chars.len(), 2);

        // fourth iteration (should be same as third, fused)
        assert_eq!(advanced_chars.next(), None);
        assert_eq!(advanced_chars.peek(), None);
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.peek_pos(), None);
        assert_eq!(advanced_chars.len(), 2);
    }
}
