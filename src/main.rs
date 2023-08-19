/* By Nova Aurora 2022. A program that quickly aproximates the final value of a large 
number of dice rolls (~99999999999999d20 is around the limit before you start losing precision.)
It does this by generating the expected distribution of the roll, then 
directly sampling the distribution, bypassing simulating the dice rolls themselves.
Think of it not as rolling 2d6 but instead 1d36, where the values are then 
mapped to the possible values of 2d6. Ironically this does not work for small numbers of dice
(They don't follow normals as closely), and so a 'slow method' is used as a backup for those.
*/
use std::env;
use rand_distr::{Normal, Distribution};
use rand::prelude::*;
fn mean(dice: f64, sides: f64) -> f64 {
    let mean: f64 = dice * ((sides + 1.) / 2.);
    return mean
}
fn rollf(dice: f64, sides: f64) -> usize {
    let mean: f64 = mean(dice, sides);
    let variance = dice * ((sides.powi(2)) - 1.) / 12.;
    let stddev = variance.sqrt();
    let normal = Normal::new(mean, stddev).unwrap();
    loop {
        let roll = normal.sample(&mut thread_rng());
        if roll >= dice && roll <= (dice * sides) {
            return roll.round() as usize;
        }
        else{
            println!("Generation error, trying again.")
        }
    }
}
fn rolln(dice: f64, sides: f64) -> usize {
    let mut result = 0;
    let die = rand::distributions::Uniform::from(1..(sides as usize + 1));
    for _ in 0..(dice as usize){
        let roll = die.sample(&mut thread_rng());
        result += roll;
    }
    return result
}
fn main() {
    let diceerr = "Couldn't find the number of dice";
    let sideerr = "Couldn't find the number of sides";
    let dice = env::args().nth(1).expect(diceerr).parse::<f64>().expect(diceerr);
    let sides = env::args().nth(2).expect(sideerr).parse::<f64>().expect(sideerr);
    if dice.is_sign_positive() && dice.is_normal() && sides.is_sign_positive() && sides.is_normal() {
        if dice > 30. {
            let r = rollf(dice, sides);
            println!("Rolled: {}", r)
        }
        else {
            let r = rolln(dice, sides);
            println!("Rolled: {}", r)
        }
    }
    else{
        println!("Invalid arguments!")
    }
}
