use std::str::CharIndices;
use std::iter::FusedIterator;

pub struct AdvancedChars<'a> {
    chars: CharIndices<'a>,
    peek_ch: Option<char>,
    peek_pos: Option<usize>,
    current_pos: Option<usize>,
    length: usize,
}

impl<'a> AdvancedChars<'a> {
    fn new(input: &str) -> AdvancedChars<'_> {
        let mut chars = input.char_indices();
        let peek_ch  = chars.next().map(|(_, c)| c);
        let current_pos = None;
        let peek_pos = if input.is_empty() {
            None
        } else {
            Some(0)
        };
        let length = input.len();
        AdvancedChars {
            chars,
            peek_ch,
            peek_pos,
            current_pos,
            length,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.peek_ch
    }

    pub fn peek_pos(&self) -> Option<usize> {
        self.peek_pos
    }

    pub fn current_pos(&self) -> Option<usize> {
        self.current_pos
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<'a> Iterator for AdvancedChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.peek_ch.take();
        self.current_pos = self.peek_pos.take();

        match self.chars.next() {
            Some((pos, ch)) => {
                self.peek_pos = Some(pos);
                self.peek_ch = Some(ch);
            }
            None => {
                self.peek_pos = None;
                self.peek_ch = None;
            }
        }

        res
    }
}

impl<'a> FusedIterator for AdvancedChars<'a> {  }

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
        assert_eq!(advanced_chars.peek(), Some('a'));
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
        assert_eq!(advanced_chars.peek(), Some('h'));
        assert_eq!(advanced_chars.peek_pos(), Some(0));
        assert_eq!(advanced_chars.current_pos(), None);
        assert_eq!(advanced_chars.len(), 2);

        // first iteration
        assert_eq!(advanced_chars.next(), Some('h'));
        assert_eq!(advanced_chars.peek(), Some('i'));
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
