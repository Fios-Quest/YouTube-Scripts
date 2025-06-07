fn main() {
    // use std::collections::LinkedList;

    let hello = String::from("Hello");
    let world = String::from("World");

    let mut ll = LinkedList::new();
    ll.push_back(hello);
    ll.push_back(world);

    let v: Vec<_> = ll.into_iter().collect();

    assert_eq!(v, vec![String::from("Hello"), String::from("World")]);

    use std::collections::LinkedList;

    let hello = String::from("Hello");
    let world = String::from("World");

    let mut ll = LinkedList::new();
    ll.push_back(hello);
    ll.push_back(world);

    assert_eq!(
        ll.into_iter().collect::<Vec<_>>(),
        vec![String::from("Hello"), String::from("World")]
    );
}
