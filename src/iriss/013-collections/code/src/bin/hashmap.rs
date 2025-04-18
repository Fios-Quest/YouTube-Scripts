fn main() {
    // use std::collections::HashMap;

    let mut hashmap: HashMap<&str, &str> = HashMap::new();

    hashmap.insert("str is hash", "Value");

    let mut hashmap_with_capacity = HashMap::with_capacity(1);

    hashmap_with_capacity.insert(String::from("String is Hash too"), 12345);

    let hashmap_from_array = HashMap::from([(12345, "i32 is also hash"), (67890, "Another value")]);

    dbg!(hashmap_from_array);

    // use std::collections::HashMap;

    let mut map = HashMap::from([("Key".to_string(), "Value".to_string())]);

    assert_eq!(map.get("Key"), Some(&"Value".to_string()));
    assert_eq!(map.get("Not a Key"), None);

    if let Some(mut value) = map.get_mut("Key") {
        value.push_str(" Changed");
    }

    assert_eq!(map.get("Key"), Some(&"Value Changed".to_string()));

    // use std::collections::HashMap;

    let mut map = HashMap::from([
        ("Existing Key".to_string(), "Value".to_string()),
    ]);

    map.entry("Existing Key".to_string())
        .and_modify(|value| value.push_str(" Changed"))
        .or_insert("Inserted Value".to_string());

    map.entry("Nonexistent Key".to_string())
        .and_modify(|value| value.push_str(" Changed"))
        .or_insert("Inserted Value".to_string());

    assert_eq!(map.get("Existing Key"), Some(&"Value Changed".to_string()));
    assert_eq!(
        map.get("Nonexistent Key"),
        Some(&"Inserted Value".to_string())
    );


    // use std::collections::HashMap;

    let key = "Key".to_string();
    let value = "Value".to_string();
    // At this point we own these 👆🏻values

    let mut map = HashMap::with_capacity(1);

    map.insert(key, value);
    // Here 👆🏻 we move ownership into the hashmap

    // Here 👇🏻 we move ownership out of the hashmap
    let recovered_value = map.remove("Key").expect("key not found");
    
    assert_eq!(&recovered_value, "Value");
    

    use std::collections::HashMap;

    let key = "Key".to_string();
    let value = "Value".to_string();
    // At this point we own these 👆🏻values

    let mut map = HashMap::with_capacity(1);

    map.insert(key, value);
    // Here 👆🏻 we move ownership into the hashmap

    // Here 👇🏻 we move ownership out of the hashmap
    let (recovered_key, recovered_value) = map
        .remove_entry("Key")
        .expect("key not found");

    assert_eq!(&recovered_key, "Key");
    assert_eq!(&recovered_value, "Value");

    // Obviously the key abd value will no longer be part of the HashMap
    assert_eq!(map.get("Key"), None);
    
}
