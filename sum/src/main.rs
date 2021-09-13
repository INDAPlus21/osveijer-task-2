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

     let n: usize = lines
        .next().unwrap()
        .parse::<usize>().unwrap();

     let mut num_vector = lines
        .next().unwrap()
        .split(" ")                         //get list of components
        .map(|x| x.parse::<u32>().unwrap()) //convert into unsigned integers
        .collect::<Vec<u32>>();             //turn map iterable into vector iterable
     
     num_vector.sort();                     //sort values from lowest to highest

     eprintln!("Numbers: {:?}", num_vector);        
     
     //determine starting element for sum
     let sum_start;
     if n%2==0 {
        sum_start = n/2;
     } else {
        sum_start = (n-1)/2;                //(n-1) because the change of sum_start has inverted effect on the amount of numbers included
     }

     //calculate sum
     let mut sum: u32 = 0;
     for i in sum_start..n {
        sum += num_vector[i];
     }
 
     println!("{}",sum);
 }
