use std::time::Instant;
use async_iterator::AsyncIterator;
// use rand::prelude::*;
use plotters::prelude::*;

#[tokio::main]
async fn main() {
    // Create a simple synchronous iterator (for demonstration purposes)
    // let mut rng = rand::thread_rng();
    let blocks = vec![1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let data: Vec<i32> = (0..1_000_000).flat_map(|_| [-1, 1]).collect();

    // println!("{} elements:", i);
    // let data: Vec<i32> = (0..i).flat_map(|_| [-1, 1]).collect();
    // Your code here: You can perform any operation with 'i' within this loop

    let start = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, x| acc+x);
    let elapsed = start.elapsed();
    println!(
        "Sequential: Elapsed time: {:?}",
        elapsed
    );

    // Loop through the specified blocks
    for block_size in blocks {

        let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
        let start = Instant::now();
        let async_fold_result: i32 = AsyncIterator::async_fold(async_iter,0, |acc, x| acc + x, block_size).await;
        let elapsed = start.elapsed();
        println!(
            "Block size: {}, Elapsed time: {:?}",
            block_size,
            elapsed.as_nanos()
        );
    }

    // Check the results
    // assert_eq!(async_fold_result, fold_result);

    // let data: Vec<i32> = vec![0; 500_000_000];
}