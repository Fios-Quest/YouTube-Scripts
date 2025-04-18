fn main() {

    let example: Vec<i32> = Vec::with_capacity(10);
    assert_eq!(example.len(), 0);
    assert!(example.capacity() >= 10);

    let example = vec![0, 1, 2, 3];
    assert_eq!(example.len(), 4);

    let mut v = Vec::with_capacity(2);
    v.push("Hello");
    v.push("World!");

    assert_eq!(v, vec!["Hello", "World!"]);

    assert_eq!(v.pop(), Some("World!"));

    assert_eq!(v, vec!["Hello"]);

    let v = vec!["Hello", "World!"];
    assert_eq!(v[0], "Hello");
    assert_eq!(v[1], "World!");
    // let _ = v[2]; // ❗️Panics❗️



    let v = vec!["Hello", "World!"];
    assert_eq!(v.get(0), Some(&"Hello"));
    assert_eq!(v.get(1), Some(&"World!"));
    assert_eq!(v.get(2), None);

    let mut v = vec!["Hello".to_string()];
    if let Some(hello) = v.get_mut(0) {
        assert_eq!(hello, &mut "Hello".to_string());
        hello.push_str(", World!");
    }
    assert_eq!(v, vec!["Hello, World!".to_string()]);

    let mut v = vec![0, 1, 2, 3, 4, 5];

    assert_eq!(v.get(2..), Some(&[2, 3, 4, 5][..]));
    assert_eq!(v.get(..2), Some(&[0, 1][..]));
    assert_eq!(v.get(6..), Some(&[][..]));
    
    if let Some(inner) = v.get_mut(2..) {
        inner[0] += 10;
    };

    assert_eq!(v, vec![0, 1, 12, 3, 4, 5]);


}
