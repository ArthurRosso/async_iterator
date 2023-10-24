use async_iterator::AsyncIterator;
use std::time::Instant;

fn main() {
    // Create a simple synchronous iterator (for demonstration purposes)
    let data = vec![1, 2, 3, 4, 5];
    let iter = data.iter().cloned();

    // Create an `AsyncIterator` from the synchronous iterator
    let async_ter = async_iter(iter);

    let start_time = Instant::now();
    // Define an asynchronous folding function (e.g., summation)
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, item| acc + item).await;
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Async computed in {:?}.", elapsed_time);

    let start_time = Instant::now();
    // Fold the original iterator synchronously
    let fold_result = data.iter().cloned().fold(0, |acc, x| acc + x);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Sequential computed in {:?}.", elapsed_time);
    
    // Check the results
    assert_eq!(async_fold_result, fold_result);
}