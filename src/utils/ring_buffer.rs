#[derive(Copy, Clone)]
pub struct RingBuffer<T: Copy, const CAPACITY: usize> {
    head: usize,
    tail: usize,
    buffer: [Option<T>; CAPACITY],
    size: usize,
}

impl<T: Copy, const CAPACITY: usize> RingBuffer<T, CAPACITY> {
    pub const fn new() -> Self {
        RingBuffer {
            head: 0,
            tail: 0,
            buffer: [None; CAPACITY],
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) -> () {
        self.buffer[self.head] = Some(value);
        self.head = (self.head + 1) % CAPACITY;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let res = self.buffer[self.tail];
        self.tail = (self.tail + 1) % CAPACITY;
        self.size -= 1;
        return res;
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }
}
