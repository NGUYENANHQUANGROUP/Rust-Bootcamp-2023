// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    let z = x;
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
    println!("{}", s);
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

    let mut additions: Vec<usize> = vec![0]; // Fix 1: Correct the type of "additions"

    println!("{:?}", values_number);

    let max_iterations = 10; // Set a maximum number of iterations for the loop to avoid infinite loop

    let mut iterations = 0; // Initialize the iteration counter

    while iterations < max_iterations && additions.len() > 0 { // Fix 2: Add a condition to stop the loop
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for &element_index in &additions { // Fix 3: Use "&" to borrow the element_index
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }

        iterations += 1; // Increment the iteration counter
    }
}


// Exercise 4
// Make it compile
fn exercise4(value: u32) -> &'static str {
    let str_value = value.to_string(); // Convert u32 to String
    static  str_ref: &str = &str_value; // Obtain a reference to the String
    str_ref // Return the reference to the String
}

use std::collections::HashMap;

fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;

    let res = match my_map.get(&key) {
        Some(child) => child.as_str(),
        None => {
            let value = "3.0";
            my_map.insert(key, value.to_string());
            value // Return a static string slice directly
        }
    };

    println!("{}", res);
}


use std::io;

fn exercise6() {
    let mut prev_key = String::new(); // Fix 1: Change the type of prev_key to String

    for line in io::stdin().lock().lines() { // Fix 2: Use io::stdin().lock() to prevent conflicts
        let s = line.unwrap();

        let data: Vec<&str> = s.split('\t').collect(); // Fix 3: Use single quotes for the tab character

        if prev_key.is_empty() { // Fix 4: Use is_empty() to check if prev_key is empty
            prev_key = data[0].to_string(); // Fix 5: Convert &str to String
        }
    }
}


fn exercise7() {
    let mut v: Vec<&str> = Vec::new();
    let s: &str;
    {
        let chars = [b'x', b'y', b'z'];
        s = std::str::from_utf8(&chars).unwrap();
        v.push(s);
    }
    println!("{:?}", v);
}


// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting = vec!["Alice", "Ben"];
    
    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person);
    }
}
