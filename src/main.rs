use std::time::Instant;
use async_iterator::AsyncIterator;

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

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, &x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);

    // Map
    let start = Instant::now();
    let map_result = data.iter().map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_map_result = async_iter.map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(map_result.fold(0, |acc, _x| acc+1), async_map_result.async_fold(0, |acc, _x| acc+1).await);


    // Filter
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let filter_result = data.iter().filter(|&&x| x > 0);
    let elapsed = start.elapsed();
    println!(
        "Filter Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_filter_result = async_iter.filter(|&&x| x>0);
    let elapsed = start.elapsed();
    println!(
        "Filter Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(filter_result.fold(0, |acc, _x| acc+1), async_filter_result.async_fold(0, |acc, _x| acc+1).await);


    // Collect
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let collect_result: Vec<_> = data.iter().collect();
    let elapsed = start.elapsed();
    println!(
        "Collect Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_collect_result = async_iter.collect_vec().await;
    let elapsed = start.elapsed();
    println!(
        "Collect Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(collect_result, async_collect_result);
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

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, &x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);

    // Map
    let start = Instant::now();
    let map_result = data.iter().map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_map_result = async_iter.map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(map_result.fold(0, |acc, _x| acc+1), async_map_result.async_fold(0, |acc, _x| acc+1).await);


    // Filter
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let filter_result = data.iter().filter(|&&x| x > 0);
    let elapsed = start.elapsed();
    println!(
        "Filter Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_filter_result = async_iter.filter(|&&x| x>0);
    let elapsed = start.elapsed();
    println!(
        "Filter Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(filter_result.fold(0, |acc, _x| acc+1), async_filter_result.async_fold(0, |acc, _x| acc+1).await);


    // Collect
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let collect_result: Vec<_> = data.iter().collect();
    let elapsed = start.elapsed();
    println!(
        "Collect Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_collect_result = async_iter.collect_vec().await;
    let elapsed = start.elapsed();
    println!(
        "Collect Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(collect_result, async_collect_result);
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

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, &x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);

    // Map
    let start = Instant::now();
    let map_result = data.iter().map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_map_result = async_iter.map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(map_result.fold(0, |acc, _x| acc+1), async_map_result.async_fold(0, |acc, _x| acc+1).await);


    // Filter
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let filter_result = data.iter().filter(|&&x| x > 0);
    let elapsed = start.elapsed();
    println!(
        "Filter Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_filter_result = async_iter.filter(|&&x| x>0);
    let elapsed = start.elapsed();
    println!(
        "Filter Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(filter_result.fold(0, |acc, _x| acc+1), async_filter_result.async_fold(0, |acc, _x| acc+1).await);


    // Collect
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let collect_result: Vec<_> = data.iter().collect();
    let elapsed = start.elapsed();
    println!(
        "Collect Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_collect_result = async_iter.collect_vec().await;
    let elapsed = start.elapsed();
    println!(
        "Collect Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(collect_result, async_collect_result);
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

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, &x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);

    // Map
    let start = Instant::now();
    let map_result = data.iter().map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_map_result = async_iter.map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(map_result.fold(0, |acc, _x| acc+1), async_map_result.async_fold(0, |acc, _x| acc+1).await);


    // Filter
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let filter_result = data.iter().filter(|&&x| x > 0);
    let elapsed = start.elapsed();
    println!(
        "Filter Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_filter_result = async_iter.filter(|&&x| x>0);
    let elapsed = start.elapsed();
    println!(
        "Filter Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(filter_result.fold(0, |acc, _x| acc+1), async_filter_result.async_fold(0, |acc, _x| acc+1).await);


    // Collect
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let collect_result: Vec<_> = data.iter().collect();
    let elapsed = start.elapsed();
    println!(
        "Collect Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_collect_result = async_iter.collect_vec().await;
    let elapsed = start.elapsed();
    println!(
        "Collect Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(collect_result, async_collect_result);
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

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_fold_result: i32 = async_iter.async_fold(0, |acc, &x| acc + x).await;
    let elapsed = start.elapsed();
    println!(
        "Fold Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(fold_result, async_fold_result);

    // Map
    let start = Instant::now();
    let map_result = data.iter().map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    let async_map_result = async_iter.map(|x| x + 1);
    let elapsed = start.elapsed();
    println!(
        "Map Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(map_result.fold(0, |acc, _x| acc+1), async_map_result.async_fold(0, |acc, _x| acc+1).await);


    // Filter
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let filter_result = data.iter().filter(|&&x| x > 0);
    let elapsed = start.elapsed();
    println!(
        "Filter Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_filter_result = async_iter.filter(|&&x| x>0);
    let elapsed = start.elapsed();
    println!(
        "Filter Async: Elapsed time: {:?}",
        elapsed
    );

    assert_eq!(filter_result.fold(0, |acc, _x| acc+1), async_filter_result.async_fold(0, |acc, _x| acc+1).await);


    // Collect
    let start = Instant::now();
    // let filter_result: i32 = data.iter().filter(|&&x| x > 0).fold(0, |acc, _x| acc+1);
    let collect_result: Vec<_> = data.iter().collect();
    let elapsed = start.elapsed();
    println!(
        "Collect Sequential: Elapsed time: {:?}",
        elapsed
    );

    let async_iter: async_iterator::Iter<'_, i32> = async_iterator::Iter{slice: &data};
    let start = Instant::now();
    // let async_filter_result = async_iter.filter(|&&x| x<0).async_fold(0, |acc, _x| acc+1).await;
    let async_collect_result = async_iter.collect_vec().await;
    let elapsed = start.elapsed();
    println!(
        "Collect Async: Elapsed time: {:?}",
        elapsed
    );

    // assert_eq!(collect_result, async_collect_result);
}