use core::mem::MaybeUninit;

#[derive(Debug, PartialEq)]
pub enum RingBufferError {
    PushAtMaxCapacity,
}

#[derive(Copy, Clone)]
pub struct RingBuffer<T: Copy, const CAPACITY: usize> {
    head: usize,
    tail: usize,
    buffer: [MaybeUninit<T>; CAPACITY],
    size: usize,
}

impl<T: Copy, const CAPACITY: usize> RingBuffer<T, CAPACITY> {
    pub const fn new() -> Self {
        RingBuffer {
            head: 0,
            tail: 0,
            buffer: [MaybeUninit::uninit(); CAPACITY],
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), RingBufferError> {
        if self.size >= CAPACITY {
            return Err(RingBufferError::PushAtMaxCapacity);
        }
        self.buffer[self.head] = MaybeUninit::new(value);
        self.head = (self.head + 1) % CAPACITY;
        self.size += 1;
        return Ok(());
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        // SAFETY: The size is non-zero usize, so the element is initialized.
        let res = unsafe { self.buffer[self.tail].assume_init() };
        self.tail = (self.tail + 1) % CAPACITY;
        self.size -= 1;
        return Some(res);
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }
}