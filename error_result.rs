use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "1a";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}

