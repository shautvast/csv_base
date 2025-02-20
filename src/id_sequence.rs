use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct ThreadSafeIdGenerator {
    counter: AtomicUsize,
}

impl ThreadSafeIdGenerator {
    pub fn new(start: usize) -> Self {
        Self {
            counter: AtomicUsize::new(start),
        }
    }

    pub fn next(&self) -> usize {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }
}
