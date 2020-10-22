use std::iter::FusedIterator;

pub struct AdvancedIter<T: Iterator> {
    iter: T,
    peek_item: Option<T::Item>,
    peek_pos: Option<usize>,
    current_pos: Option<usize>,
}

impl<T: Iterator> AdvancedIter<T> {
    fn new(mut iter: T) -> AdvancedIter<T> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
