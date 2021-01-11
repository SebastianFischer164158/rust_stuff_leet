pub fn find_numbers(nums: Vec<i32>) -> i32{
    let mut counter = 0;
    for digit in nums.iter() {
        // let string_of_digit: String = digit.to_string();
        // println!("the value is: {}", string_of_digit.len())
        if (digit.to_string()).len() % 2 == 0 {
            counter += 1;
        }
    }
    return counter
}