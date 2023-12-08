use crate::days::day_eight::{day_eight_part_one, day_eight_part_two};
use crate::days::day_five::{day_five_part_one, day_five_part_two};
use crate::days::day_four::{day_four_part_one, day_four_part_two};
use crate::days::day_one::day_one_part_two;
use crate::days::day_six::{day_six_part_one, day_six_part_two};
use crate::days::day_three::{day_three_part_one, day_three_part_two};

mod days;

fn main() {
    println!("Result day_one_part_two   = {}", day_one_part_two());
    println!("Result day_three_part_one = {}", day_three_part_one());
    println!("Result day_three_part_two = {}", day_three_part_two());
    println!("Result day_four_part_one  = {}", day_four_part_one());
    println!("Result day_four_part_two  = {}", day_four_part_two());
    println!("Result day_five_part_one  = {:?}", day_five_part_one());
    println!("Result day_five_part_two  = {:?}", day_five_part_two());
    println!("Result day_six_part_one   = {:?}", day_six_part_one());
    println!("Result day_six_part_two   = {:?}", day_six_part_two());
    println!("Result day_eight_part_one = {:?}", day_eight_part_one());
    println!("Result day_eight_part_two = {:?}", day_eight_part_two());
}
