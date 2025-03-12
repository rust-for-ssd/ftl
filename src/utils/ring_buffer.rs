
use core::mem::MaybeUninit;

pub struct RingBuffer <T:Copy, const SIZE: usize> {
    head: usize, 
    tail: usize,
    buffer: [Option<T>; SIZE]
}

impl <T:Copy, const SIZE: usize> RingBuffer <T, SIZE> {
    fn new () -> Self {
        RingBuffer{
            head: 0, 
            tail: 0,
            buffer: [None; SIZE]
        }
    }
    fn push(&mut self, value: T) -> () {
        self.buffer[self.head] = Some(value); 
        self.head = self.head + 1; 
    }
    fn pop() {todo!()}
}
