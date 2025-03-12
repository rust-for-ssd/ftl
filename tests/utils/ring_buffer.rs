use ftl::utils::ring_buffer::RingBuffer;

#[test_case]
pub fn new_rb() {
    let mut rb = RingBuffer::<i32, 8>::new();
    assert_eq!(rb.get_size(), 0);
    assert_eq!(rb.pop(), None);
}

#[test_case]
pub fn push_pop_size() {
    let mut rb = RingBuffer::<i32, 8>::new();
    assert_eq!(rb.get_size(), 0);
    assert_eq!(rb.pop(), None);
    rb.push(1);
    rb.push(3);
    rb.push(5);
    assert_eq!(rb.get_size(), 3);
    assert_eq!(rb.pop(), Some(1));
    assert_eq!(rb.get_size(), 2);
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.get_size(), 1);
    assert_eq!(rb.pop(), Some(5));
    assert_eq!(rb.get_size(), 0);
    assert_eq!(rb.pop(), None);
    assert_eq!(rb.get_size(), 0);
    assert_eq!(rb.pop(), None);
}
