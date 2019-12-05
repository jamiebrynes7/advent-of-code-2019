use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const START: u32 = 145852;
const END: u32 = 616942;

fn main() -> Result<()> {
    let count = (START..END).filter(|num| matches_requirement(*num)).count();
    println!("{}", count);

    let count = (START..END).filter(|num| matches_requirements_part_2(*num)).count();
    println!("{}", count);

    println!("{}", matches_requirements_part_2(123444));
    println!("{}", matches_requirements_part_2(111122));

    Ok(())
}

// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
fn matches_requirement(mut number: u32) -> bool {

    let mut has_double = false;
    let mut is_increasing = true;
    let mut last_digit = None;

    let mut divisor = 100000;
    for _ in 0..6 {
        let digit = number / divisor;
        if let Some(last) = last_digit {
            if digit == last {
                has_double = true;
            }
            if last > digit {
                is_increasing = false;
            }
        }

        last_digit = Some(digit);
        number = number % divisor;
        divisor /= 10;
    }

    has_double && is_increasing
}


fn matches_requirements_part_2(mut number: u32) -> bool {
    let mut has_double = false;
    let mut in_a_row = 1;
    let mut is_increasing = true;
    let mut last_digit = None;

    let mut divisor = 100000;
    for _ in 0..6 {
        let digit = number / divisor;
        if let Some(last) = last_digit {
            if digit == last {
                in_a_row += 1;
            }
            else {
                if in_a_row == 2 {
                    has_double = true;
                }

                in_a_row = 1;
            }

            if last > digit {
                is_increasing = false;
            }
        }

        last_digit = Some(digit);
        number = number % divisor;
        divisor /= 10;
    }

    if in_a_row == 2 {
        has_double = true;
    }

    has_double && is_increasing
}