use std::time::Instant;
use async_iterator::AsyncIterator;

#[tokio::main]
async fn main() {
    let data: Vec<i32> = (0..10).flat_map(|_| [-1, 1]).collect();
    

    // let start = Instant::now();
    // let fold_result: i32 = data.iter().fold(0, |acc, x| acc+x);
    // let elapsed = start.elapsed();
    // println!(
    //     "Sequential: Elapsed time: {:?}",
    //     elapsed
    // );

    // let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    // let start = Instant::now();
    // let async_fold_result: i32 = AsyncIterator::async_fold(async_iter,0, |acc, x| acc + x).await;
    // let elapsed = start.elapsed();
    // println!(
    //     "Async: Elapsed time: {:?}",
    //     elapsed
    // );

    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let filter_result: Vec<_> = data.iter().filter(|&&x| x > 0).collect();
    let elapsed = start.elapsed();
    println!(
        "Sequential: Elapsed time: {:?}",
        elapsed
    );
    println!(
        "Result: {:?}",
        filter_result
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_filter_result: Vec<_> = async_iter.filter(|&&x| x<0).collect_vec().await;
    let elapsed = start.elapsed();
    println!(
        "Async: Elapsed time: {:?}",
        elapsed
    );
    println!(
        "Result: {:?}",
        async_filter_result
    );
    

    // Check the results
    assert_eq!(filter_result, async_filter_result);
}