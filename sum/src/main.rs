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
 
     /* add code here ... */

     let n: usize = lines
        .next().unwrap()
        .parse::<usize>().unwrap();

     let mut num_vector = lines
        .next().unwrap()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
     
     num_vector.sort();

     eprintln!("Numbers: {:?}", num_vector);

     let sum_start;
     if n%2==0 {
        sum_start = n/2;
     } else {
        sum_start = (n+1)/2;
     }

     let mut sum: u32 = 0;
     for i in sum_start-1..n {
        sum += num_vector[i];
     }
 
     println!("{}",sum);
 }
