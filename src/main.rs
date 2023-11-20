use std::time::Instant;

use async_iterator::AsyncIterator;
// use rand::prelude::*;


#[tokio::main]
async fn main() {
    // Create a simple synchronous iterator (for demonstration purposes)
    // let mut rng = rand::thread_rng();

    let data: Vec<i32> = (0..500_000_000).flat_map(|_| [-1, 1]).collect();

    // let async_iter = async_iterator::async_iter(data.iter().cloned());

    // let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};

    // let start_time = Instant::now();
    // let async_fold_result: i32 = AsyncIterator::async_fold(async_iter,0, |acc, item| acc + item).await;
    // println!("Async computed in {:?}.", start_time.elapsed());

    let seq_time = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, x| acc+x);
    println!("Sequential computed in {:?}.", seq_time.elapsed());

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};

    let async_time = Instant::now();
    let async_fold_result: i32 = AsyncIterator::async_fold(async_iter,0, |acc, x| acc + x).await;
    println!("Async computed in {:?}.", async_time.elapsed());
    
    // Check the results
    assert_eq!(async_fold_result, fold_result);
}