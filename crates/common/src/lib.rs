use std::fmt;
use std::iter::FusedIterator;

use log::info;

/// Advanced iter is and iterator that is advanced one. It is like Peekable<T> except the peek item
/// is already advanced.
pub struct AdvancedIter<T: Iterator> {
    iter: T,
    peek_item: Option<T::Item>,
    peek_pos: Option<usize>,
    current_pos: Option<usize>,
}

impl<T: Iterator> AdvancedIter<T> {
    pub fn new(mut iter: T) -> AdvancedIter<T> {
        let peek_item = iter.next();
        let current_pos = None;
        let peek_pos = Some(0);
        AdvancedIter {
            iter,
            peek_item,
            peek_pos,
            current_pos,
        }
    }

    pub fn peek_pos(&self) -> Option<usize> {
        self.peek_pos
    }

    pub fn current_pos(&self) -> Option<usize> {
        self.current_pos
    }

    pub fn peek_item(&self) -> Option<&<Self as Iterator>::Item> {
        self.peek_item.as_ref()
    }
}

impl<T: Iterator> Iterator for AdvancedIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.peek_item.take();
        self.current_pos = self.peek_pos;

        match self.iter.next() {
            Some(item) => {
                self.peek_pos = Some(self.peek_pos.unwrap() + 1);
                self.peek_item = Some(item);
            }
            None => {
                self.peek_pos = None;
                self.peek_item = None;
            }
        }

        res
    }
}

pub trait Peekable: Iterator {
    fn peek(&self) -> Option<&Self::Item>;
}

impl<T: Iterator> Peekable for AdvancedIter<T> {
    fn peek(&self) -> Option<&Self::Item> {
        self.peek_item.as_ref()
    }
}

impl<T: Iterator> FusedIterator for AdvancedIter<T> {  }

pub trait Accept<T: PartialEq + fmt::Debug>: Iterator<Item = T> + Peekable {

    fn accept(&mut self, valid: Self::Item) -> bool {
        match self.peek() {
            Some(c) if c == &valid => {
                info!("char `{:?}` is accepted", c);
                self.next();
                true
            },
            _ => {
                info!("char `{:?}` is not accepted", self.peek());
                false
            },
        }
    }

    /// Accepts while predicate returns true. Does not accept the char the predicate returns
    /// false for.
    fn accept_while(&mut self, predicate: impl Fn(&Self::Item) -> bool) {
        while let Some(c) = self.peek() {
            if !predicate(&c) {
                info!("char `{:?}` is not accepted", c);
                break;
            } else {
                info!("char `{:?}` is accepted", c);
                self.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advanced_iter_simple() {
        let chars = "hi".chars();
        let mut advanced_iter = AdvancedIter::new(chars);
        assert_eq!(advanced_iter.next(), Some('h'));
    }
}
