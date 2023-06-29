#![allow(dead_code, unused_imports, unused_variables)]

use rayon::prelude::*;

fn main() {
    let ids = vec![" 1234", "5678", "-9012", "ten", "3456", "7890"];

    // ? Sequential execution
    let seq_ids = ids
        .iter()
        .map(|id| id.trim())
        .filter_map(|id| id.parse().ok())
        .filter(|num| num >= &1000)
        .collect::<Vec<usize>>();

    // NOTE Parallel execution with `par_iter`, the rest of the code is exactly the same
    let mut par_ids = ids
        .par_iter()
        .map(|id| id.trim())
        .filter_map(|id| id.parse().ok())
        .filter(|num| num >= &1000)
        .collect::<Vec<usize>>();

    // ? Parallel sorting with `par_sort`
    par_ids.par_sort();

    for id in &par_ids {
        println!("after sort: {}", id);
    }

    // ? Will throw an error because `par_iter` cannot be directly iterated over
    // for id in par_ids.par_iter() {
    //     println!("{}", id);
    // }

    // NOTE To iterate over a parallel collection, use `for_each`
    par_ids.par_iter().for_each(|id| {
        // ? As the threads execute non-deterministically, the output may not be in order
        println!("for_each: {}", id);
    });
}
