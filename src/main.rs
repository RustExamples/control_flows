fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;

    // while could be used to loop through collections
    // but it has performance hit due to additional conditional checks
    // error prone if we access no existing index
    while index < 5 {
        println!("value is {}", arr[index]);
        index = index + 1;
    }

    // for only loops elements in array
    // for is the most used of all loops
    for element in arr.iter() {
        println!("value is {}", element);
    }

    // can also be used in place of "while" like loop certain numbers
    // (1..4) is a "Range" provided by standard library
    for i in (1..4).rev() {
        println!("value is {}", i);
    }
}