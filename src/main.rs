fn main() {
    let response: i32 = sum_numbers(5, 10);
    println!("Response is: {response}");
}

fn sum_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}
