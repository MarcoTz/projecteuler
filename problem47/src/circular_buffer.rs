use core::array;
use std::fmt;

#[derive(Debug)]
pub struct CircularBuffer<T, const N: usize> {
    contents: [T; N],
}

impl<T, const N: usize> CircularBuffer<T, N> {
    pub fn new() -> CircularBuffer<T, N>
    where
        T: Default,
    {
        CircularBuffer {
            contents: array::from_fn(|_| T::default()),
        }
    }

    pub fn push(&mut self, new_t: T) {
        self.contents.rotate_right(1);
        self.contents[0] = new_t;
    }

    pub fn all(&self, predicate: impl Fn(&T) -> bool) -> bool {
        self.contents.iter().all(predicate)
    }
}

impl<T: fmt::Display, const N: usize> fmt::Display for CircularBuffer<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.contents
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
