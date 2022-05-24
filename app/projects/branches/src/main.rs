fn main() {
    println!("----- if -----");
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("----- loop -----");

    let mut i: u32 = 0;
    loop {
        i = i + 1;
        println!("{}", i);

        if i == 5 {
            break;
        }
    }

    println!("----- while -----");

    let mut number: u32 = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("----- for iter -----");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }

    println!("----- for iter -----");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
