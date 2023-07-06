// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = &x;
    let z = &x;
}

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    //println!("{}", s);
    s
}

// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while !additions.is_empty() {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for &element_index in &additions {
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) ->  String {
    let str_value = value.to_string(); // Convert u32 to String
    str_value
}

// Exercise 5
// Make it compile
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;
    let mut value= String::new();
    let res = match my_map.get(&key) {
        Some(child) => child,
        None => {
            value = "3.0".to_string();
            my_map.insert(key, value.clone());
            &value // HERE IT FAILS
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;

fn exercise6() {
    let mut prev_key: String = String::new();

    for line in io::stdin().lines() {
        let s = line.unwrap();

        let data: Vec<String> = s.split('\t').map(|x| x.to_string()).collect();
        if prev_key.is_empty() {
            prev_key = data[0].clone();
        }
    }
}

// Exercise 7
// Make it compile
fn exercise7() {
    let chars: [u8; 3] = [b'x', b'y', b'z'];
    let mut v: Vec<&str> = Vec::new();
    {
        let s: &str = std::str::from_utf8(&chars).unwrap();
        v.push(&s);
    }
    println!("{:?}", v);
}

// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting = vec!["Alice".to_string(), "Ben".to_string()];
    
    loop {
        let mut add_input = String::new();

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<String> = add_input.trim().split_whitespace().map(|x| x.to_string()).collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0].clone();
        accounting.push(person);

    }
}
