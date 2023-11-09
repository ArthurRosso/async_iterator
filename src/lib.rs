use tokio::task::yield_now;

pub struct AsyncIterator<I>
where
    I: Iterator,
{
    base: I,
}

pub fn async_iter<I>(iter: I) -> AsyncIterator<I>
where
    I: Iterator,
{
    AsyncIterator {
        base: iter
    }
}

impl<I:Iterator> AsyncIterator<I>  {
    pub async fn async_fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, I::Item) -> B,
    {
        let mut res = init;

        loop {
            let i = self.base.by_ref().take(10_000_000);
            res = i.fold(res, &mut f);
            if let Some(next_one) = self.base.next() {
                res = f(res, next_one);
            } else {
                return res;
            }
            yield_now().await;
        }
    }
}

#[cfg(test)]
mod tests {
    // Import the necessary items from the parent module
    use super::*;

    // Write your test functions
    #[tokio::test]
    async fn test_async_fold() {
        // Create a simple synchronous iterator (for demonstration purposes)
        let data = vec![1, 2, 3, 4, 5];
        let iter = data.into_iter();
        
        // Create an `AsyncIterator` from the synchronous iterator
        let async_iter = async_iter(iter);
    
        // Define an asynchronous folding function (e.g., summation)
        let async_fold_result: i32 = async_iter.async_fold(0, |acc, item| acc + item).await;
    
        // Check the result
        assert_eq!(async_fold_result, 15);
    }
}
