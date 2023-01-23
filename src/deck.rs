use rand::Rng;
use std::collections::{vec_deque::IntoIter, VecDeque};

#[derive(Debug)]
pub struct Deck<T>(pub(crate) VecDeque<T>);

impl<T> Deck<T> {
    pub fn new_empty() -> Self {
        Self(VecDeque::new())
    }

    pub fn discard_to_top(&mut self, elem: T) {
        self.0.push_back(elem);
    }

    pub fn discard_to_bottom(&mut self, elem: T) {
        self.0.push_front(elem);
    }

    pub fn draw_from_top(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    pub fn draw_from_bottom(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

        let mut i = self.0.len();
        while i >= 2 {
            // invariant: elements with index >= i have been locked in place.
            i -= 1;
            // lock element i in place.
            self.0.swap(i, rng.gen_range(0..i + 1));
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }
}

impl<T> IntoIterator for Deck<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<Vec<T>> for Deck<T> {
    fn from(input: Vec<T>) -> Self {
        Self(VecDeque::from(input))
    }
}

impl<T> From<VecDeque<T>> for Deck<T> {
    fn from(input: VecDeque<T>) -> Self {
        Self(input)
    }
}
