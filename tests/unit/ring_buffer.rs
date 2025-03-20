use ftl::config::TOTAL_BLOCKS;
use ftl::utils::ring_buffer::RingBuffer;
use ftl::utils::ring_buffer::RingBufferError;

#[test_case]
pub fn initializes_new_ring_buffer() {
    let mut rb = RingBuffer::<i32, 8>::new();
    assert_eq!(rb.get_size(), 0);
    assert_eq!(rb.pop(), None);
}

#[test_case]
pub fn push_pop_size() {
    let mut rb = RingBuffer::<i32, 8>::new();
    assert_eq!(rb.get_size(), 0);
    assert_eq!(rb.pop(), None);
    let _ = rb.push(1);
    let _ = rb.push(3);
    let _ = rb.push(5);
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

#[test_case]
pub fn cannot_push_more_than_capacity() {
    let mut rb = RingBuffer::<i32, 8>::new();
    let _ = rb.push(1);
    let _ = rb.push(2);
    let _ = rb.push(3);
    let _ = rb.push(4);
    let _ = rb.push(5);
    let _ = rb.push(6);
    let _ = rb.push(7);
    let _ = rb.push(8);
    let res = rb.push(9);
    assert_eq!(res, Err(RingBufferError::PushAtMaxCapacity))
}

#[test_case]
pub fn can_push_to_max_cap_and_pop() {
    let mut rb = RingBuffer::<i32, 8>::new();
    let _ = rb.push(1);
    let _ = rb.push(2);
    let _ = rb.push(3);
    let _ = rb.push(4);
    let _ = rb.push(5);
    let _ = rb.push(6);
    let _ = rb.push(7);
    let _ = rb.push(8);
    rb.pop();
    let res = rb.push(9);
    assert_eq!(res, Ok(()))
}

#[test_case]
pub fn can_make_huge_ring_buffer() {
    let mut rb = RingBuffer::<usize, { TOTAL_BLOCKS }>::new();
    for i in 0..TOTAL_BLOCKS {
        assert_eq!(rb.push(i), Ok(()));
    }
}

#[test_case]
pub fn iterate_through_ring_buffer() {
    let mut rb = RingBuffer::<i32, 5>::new();
    let test_values = [10, 20, 30, 40];

    for val in &test_values {
        let _ = rb.push(*val);
    }

    let mut values = [0; 4];

    for i in 0..values.len() {
        let Some(val) = rb.pop() else {
            values[i] = -1;
            return;
        };
        values[i] = val
    }
    assert_eq!(values, test_values);
    assert_eq!(rb.get_size(), 0);
}

#[test_case]
pub fn wraparound() {
    let mut rb = RingBuffer::<i32, 3>::new();

    let _ = rb.push(1);
    let _ = rb.push(2);
    let _ = rb.push(3);

    assert_eq!(rb.pop(), Some(1));

    let _ = rb.push(4);

    assert_eq!(rb.get_size(), 3);
    assert_eq!(rb.pop(), Some(2));
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.pop(), Some(4));
}

#[test_case]
pub fn multiple_wraparounds() {
    let mut rb = RingBuffer::<i32, 3>::new();

    // First fill
    let _ = rb.push(1);
    let _ = rb.push(2);
    let _ = rb.push(3);
    assert_eq!(rb.pop(), Some(1));
    assert_eq!(rb.pop(), Some(2));

    // Cycle 1
    let _ = rb.push(4);
    let _ = rb.push(5);
    assert_eq!(rb.get_size(), 3);
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.pop(), Some(4));
    assert_eq!(rb.pop(), Some(5));

    // Cycle 2
    let _ = rb.push(6);
    let _ = rb.push(7);
    let _ = rb.push(8);
    assert_eq!(rb.pop(), Some(6));

    let _ = rb.push(9);
    assert_eq!(rb.get_size(), 3);

    // Final content
    assert_eq!(rb.pop(), Some(7));
    assert_eq!(rb.pop(), Some(8));
    assert_eq!(rb.pop(), Some(9));
}

#[test_case]
pub fn size_one_buffer() {
    let mut rb = RingBuffer::<i32, 1>::new();

    assert_eq!(rb.push(1), Ok(()));
    assert_eq!(rb.push(2), Err(RingBufferError::PushAtMaxCapacity));

    assert_eq!(rb.pop(), Some(1));
    assert_eq!(rb.push(3), Ok(()));
    assert_eq!(rb.pop(), Some(3));

    assert_eq!(rb.get_size(), 0);
    assert_eq!(rb.pop(), None);
}

#[test_case]
pub fn size_zero_buffer_should_not_be_pushable() {
    let mut rb = RingBuffer::<i32, 0>::new();
    assert_eq!(rb.push(1), Err(RingBufferError::PushAtMaxCapacity));
}
