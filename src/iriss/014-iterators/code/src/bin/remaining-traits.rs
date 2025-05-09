use std::collections::LinkedList;

fn main() {
    let mut ll = LinkedList::new();
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);

    let iter = ll.into_iter();
    assert_eq!(iter.len(), 3);

    let mut rev = iter.rev();
    assert_eq!(rev.next(), Some(3));
    assert_eq!(rev.next(), Some(2));
    assert_eq!(rev.next(), Some(1));
    assert_eq!(rev.next(), None);
}
