fn main() {

    // use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();

    heap.push('A');
    heap.push('C');
    heap.push('B');

    assert_eq!(heap.pop(), Some('C'));
    assert_eq!(heap.pop(), Some('B'));
    assert_eq!(heap.pop(), Some('A'));
    assert_eq!(heap.pop(), None);

    
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let mut heap = BinaryHeap::new();

    heap.push(Reverse('A'));
    heap.push(Reverse('C'));
    heap.push(Reverse('B'));

    assert_eq!(heap.pop(), Some(Reverse('A')));

    assert_eq!(heap.pop().expect("heap was empty").0, 'B');

    if let Some(Reverse(c)) = heap.pop() {
        assert_eq!(c, 'C');
    }

}
