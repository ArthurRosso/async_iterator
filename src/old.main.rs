use std::time::Instant;

#[tokio::main]
async fn main() {
    println!("Running test 1 - 1_024 elements");
    test1().await; // 1_024
    println!("Running test 2 - 1_000_000 elements");
    test2().await; // 1_000_000
    println!("Running test 3 - 10_000_000 elements");
    test3().await; // 10_000_000
    println!("Running test 4 - 100_000_000 elements");
    test4().await; // 100_000_000
    println!("Running test 5 - 1_000_000_000 elements");
    test5().await; // 1_000_000_000
}

async fn test1(){
    let data: Vec<i32> = (0..1_024).flat_map(|_| [-1, 1]).collect();
    
    // Experiments:

    // Fold:
    let start = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, &x| acc+x);
    let elapsed = start.elapsed();
    println!(
        "Fold Sequential: Elapsed time: {:?}",
        elapsed
    );

    let iter = data.iter().cloned();

    // Create an `AsyncIterator` from the synchronous iterator
    let async_iter = async_iterator::async_iter(iter);
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);
}

async fn test2(){
    let data: Vec<i32> = (0..1_000_000).flat_map(|_| [-1, 1]).collect();
    
    // Experiments:

    // Fold:
    let start = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, &x| acc+x);
    let elapsed = start.elapsed();
    println!(
        "Fold Sequential: Elapsed time: {:?}",
        elapsed
    );

    let iter = data.iter().cloned();

    // Create an `AsyncIterator` from the synchronous iterator
    let async_iter = async_iterator::async_iter(iter);
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);
}


async fn test3(){
    let data: Vec<i32> = (0..10_000_000).flat_map(|_| [-1, 1]).collect();
    
    // Experiments:

    // Fold:
    let start = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, &x| acc+x);
    let elapsed = start.elapsed();
    println!(
        "Fold Sequential: Elapsed time: {:?}",
        elapsed
    );

    let iter = data.iter().cloned();

    // Create an `AsyncIterator` from the synchronous iterator
    let async_iter = async_iterator::async_iter(iter);
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);
}

async fn test4(){
    let data: Vec<i32> = (0..100_000_000).flat_map(|_| [-1, 1]).collect();
    
    // Experiments:

    // Fold:
    let start = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, &x| acc+x);
    let elapsed = start.elapsed();
    println!(
        "Fold Sequential: Elapsed time: {:?}",
        elapsed
    );

    let iter = data.iter().cloned();

    // Create an `AsyncIterator` from the synchronous iterator
    let async_iter = async_iterator::async_iter(iter);
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);
}


async fn test5(){
    let data: Vec<i32> = (0..1_000_000_000).flat_map(|_| [-1, 1]).collect();
    
    // Experiments:

    // Fold:
    let start = Instant::now();
    let fold_result: i32 = data.iter().fold(0, |acc, &x| acc+x);
    let elapsed = start.elapsed();
    println!(
        "Fold Sequential: Elapsed time: {:?}",
        elapsed
    );

    let iter = data.iter().cloned();

    // Create an `AsyncIterator` from the synchronous iterator
    let async_iter = async_iterator::async_iter(iter);
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);
}