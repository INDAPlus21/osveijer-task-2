
/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());

    let dimensions = lines
       .next().unwrap()
       .split(" ")
       .map(|x| x.parse::<f32>().unwrap())
       .collect::<Vec<f32>>();
    
    let r = dimensions[0];
    let k = dimensions[1];

    let rmid: f32= (r+1.0)/2.0;
    let kmid: f32= (k+1.0)/2.0;

    let mut rect: String = "".to_owned();
    for i in 1..r as i32+1 {
        for j in 1..k as i32+1 {
            let rdist = (rmid - (i as f32 - rmid).abs()) as i32;
            let kdist = (kmid - (j as f32 - kmid).abs()) as i32;
            if rdist > 9 && kdist > 9 {
                rect = rect + ".";
            } else if rdist < kdist {
                rect = rect + &rdist.to_string();
            } else {
                rect = rect + &kdist.to_string();
            }
        }
        rect = rect + "\n"
    }

    println!("{}", rect);
}