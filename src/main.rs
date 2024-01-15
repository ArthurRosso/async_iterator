use std::time::Instant;
use async_iterator::AsyncIterator;
// use rand::prelude::*;
// use plotters::prelude::*;

#[tokio::main]
async fn main() {
    let data: Vec<i32> = (0..1_000_000_000).flat_map(|_| [-1, 1]).collect();

    let start = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, x| acc+x);
    let elapsed = start.elapsed();
    println!(
        "Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_fold_result: i32 = AsyncIterator::async_fold(async_iter,0, |acc, x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Async: Elapsed time: {:?}",
        elapsed
    );
    

    // Check the results
    assert_eq!(async_fold_result, fold_result);
}