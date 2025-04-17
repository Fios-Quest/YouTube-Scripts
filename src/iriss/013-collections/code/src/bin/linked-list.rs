fn main() {

    use std::collections::LinkedList;

    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);

    let mut right = list.split_off(2);
    list.push_back(2);
    list.append(&mut right);

    let v: Vec<_> = list.iter().copied().collect();
    assert_eq!(v, &[0, 1, 2, 3, 4, 5]);


}
