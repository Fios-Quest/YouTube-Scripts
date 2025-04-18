fn main() {

    use std::collections::BTreeMap;

    let mut map = BTreeMap::from([
        ("C Key".to_string(), "Value 1".to_string()),
        ("A Key".to_string(), "Value 2".to_string()),
        ("D Key".to_string(), "Value 3".to_string()),
        ("B Key".to_string(), "Value 4".to_string()),
    ]);

    // Get references to the first or last key/values according to Ord
    assert_eq!(map.first_key_value(), Some((&"A Key".to_string(), &"Value 2".to_string())));
    assert_eq!(map.last_key_value(), Some((&"D Key".to_string(), &"Value 3".to_string())));

    // There are also methods that return `Entry`s so you can insert of modify as necessary.
    map.first_entry().expect("Map had no entries").into_mut().push_str(" Modified First");
    map.last_entry().expect("Map had no entries").into_mut().push_str(" Modified Last");

    // Finally you can pop from the "front" (first) and "back" (last) of a BTreeMap
    assert_eq!(map.pop_first(), Some(("A Key".to_string(), "Value 2 Modified First".to_string())));
    assert_eq!(map.pop_last(), Some(("D Key".to_string(), "Value 3 Modified Last".to_string())));
    assert_eq!(map.pop_first(), Some(("B Key".to_string(), "Value 4".to_string())));
    assert_eq!(map.pop_last(), Some(("C Key".to_string(), "Value 1".to_string())));

}
