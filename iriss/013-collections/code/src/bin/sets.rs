fn main() {

    use std::collections::HashSet;

    let mut hash_set = HashSet::with_capacity(3);

    hash_set.insert("B");
    hash_set.insert("A");
    hash_set.insert("C");

    assert_eq!(hash_set.get("A"), Some(&"A"));

    use std::collections::BTreeSet;

    let mut b_tree_set = BTreeSet::new();

    b_tree_set.insert("B");
    b_tree_set.insert("A");
    b_tree_set.insert("C");

    assert_eq!(b_tree_set.get("A"), Some(&"A"));

    assert_eq!(b_tree_set.first(), Some(&"A"));
    assert_eq!(b_tree_set.last(), Some(&"C"));

}
