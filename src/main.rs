fn main() {
    let print_hello = true;

    // Blocks are also called as "arms"
    if print_hello {
        println!("Hello, world!");
    }
    else {
        println!("Hello, person!");
    }

    let number = 10;

    if number < 10 {
        println!("Lesser");
    }
    else if number > 10 {
        println!("Greater");
    }
    else {
        println!("Equal");
    }

    // Since "if" is an "expression", it can be used right side of a "statement"
    // But all blocks should return same type
    let value = if number < 10 {
        5
    } else if number > 10 {
        6
    } else {
        7
    };

    println!("value is {}", value);
}
