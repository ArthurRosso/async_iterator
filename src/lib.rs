use tokio::task::yield_now;
use async_trait::async_trait;

pub struct Iter<'a, T:'a> {
    pub slice: &'a [T]
}

#[async_trait]
pub trait AsyncIterator: IntoIterator {
    fn split_at(self, index: usize) -> (Self, Self) where Self: Sized;
    fn size_hint(&self) -> usize where Self: Sized;

    async fn async_fold<B, F>(mut self, init: B, f: F) -> B
    where
        Self: Sized,
        Self::Item : Send,
        Self::IntoIter: Send,
        B: Send,
        F: FnMut(B, Self::Item) -> B + Send + Copy,
    {
        let mut res = init;

        loop {
            let (left, right) = self.split_at(1_000_000_000);
            let start = std::time::Instant::now();
            res = left.into_iter().fold(res, f);
            eprintln!("folding took {:?}", start.elapsed());
            if right.size_hint() == 0 {
                return res;
            }

            self = right;
            yield_now().await;
        }
    }
}


impl<'a, T: 'a> IntoIterator for Iter<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.slice.into_iter()
    }
}

impl<'a, T: 'a> AsyncIterator for Iter<'a, T> {
    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.slice.split_at(index.min(self.slice.len()));
        (
            Iter { slice: left },
            Iter { slice: right }
        )
    }

    fn size_hint(&self) -> usize {
        self.slice.len()
    }
}

// #[cfg(test)]
// mod tests {

//     // Import the necessary items from the parent module
//     use super::*;

//     // Write your test functions
//     #[tokio::test]
//     async fn test_async_fold() {
//         // Create a simple synchronous iterator (for demonstration purposes)
//         let data = vec![1, 2, 3, 4, 5];
//         let iter = data.into_iter();
        
//         // Create an `AsyncIterator` from the synchronous iterator
//         let async_iter = async_iter(iter);
    
//         // Define an asynchronous folding function (e.g., summation)
//         let async_fold_result: i32 = async_iter.async_fold(0, |acc, item| acc + item).await;
    
//         // Check the result
//         assert_eq!(async_fold_result, 15);
//     }
// }
