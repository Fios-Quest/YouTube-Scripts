fn main() {

    let a1 = [0, 1, 1 + 1, 9 / 3];
    assert_eq!(a1, [0, 1, 2, 3]);

    let a2 = [1 + 2; 5];
    assert_eq!(a2, [3, 3, 3, 3, 3]);

    // let arr = [0, 1, 2, 3, 4];
    // for i in arr {
    //     println!("{i}");
    // }
    //
    // let arr = [0, 1, 2, 3, 4];
    // println!("{}", arr[5]);
    
    let arr = [0, 1, 2, 3, 4];
    assert_eq!(arr.get(4), Some(&4));
    assert_eq!(arr.get(5), None);

    let mut arr = [10, 11, 12, 13, 14];
    assert_eq!(arr.get(4), Some(&14));
    if let Some(value) = arr.get_mut(4) {
        *value += 10;
    }
    assert_eq!(arr, [10, 11, 12, 13, 24]);

    let a1 = [1, 2, 3, 4, 5];
    pass_an_array(arr);
    // Because i32 is Copy, we still own this data
    println!("{a1:?}");

    let a2 = [1, 2, 3];
    // pass_an_array(a2);

}

fn pass_an_array(arr: [i32; 5]) {
    println!("{arr:?}");
}


