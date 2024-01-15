use tokio::task::yield_now;
use async_trait::async_trait;

pub struct Iter<'a, T:'a> {
    pub slice: &'a [T]
}

pub struct Map<I, O> {
    base: I,
    op: O,
}

pub struct MapFolder<F, O> {
    folder: F,
    op: O,
}

pub struct BasicFolder<B, O> {
    init: B,
    op: O,
}

pub trait Folder<Item> {
    type B;
    fn fold<I:Iterator<Item=Item>>(self, i: I) -> Self;
    fn into_result(self) -> Self::B;
}

impl<Item, B, O> Folder<Item> for BasicFolder<B, O> where 
O: Fn(B, Item) -> B {
    type B = B;
    fn fold<I: Iterator<Item=Item>>(mut self, i:I) -> Self {
        let new_b = i.fold(self.init, &self.op);
        self.init = new_b;
        self
    }
    
    fn into_result(self) -> Self::B {
        self.init
    }
}

impl<Item, R, F, O> Folder<Item> for MapFolder<F, O> where
F: Folder<R>,
O: Fn(Item) -> R {
    type B = F::B;
    fn fold<I: Iterator<Item=Item>>(mut self, i:I) -> Self {
        let (base, op) = (self.folder, self.op);
        let new_base_folder = base.fold(i.map(&op));
        self.folder = new_base_folder;
        self.op = op;
        self
    }

    fn into_result(self) -> Self::B {
        self.folder.into_result()
    }

}

#[async_trait]
pub trait AsyncIterator: Sized { 
    type Item;
    async fn fold_folder<F:Folder<Self::Item>>(self, folder:F) -> F::B
    where
    Self: Send,
    F: Send;

    // on recupere l'op du fold donne en arg (et le init)
    // on les mets dans un BasicFolder qui implement le trait Folder
    // on appelle self.fold_folder(basic)
    async fn async_fold<B, F>(mut self, init: B, f: F) -> B
    where
        Self::Item : Send,
        B: Send,
        F: Fn(B, Self::Item) -> B + Send + Copy,
    {
        let basic_folder = BasicFolder { init, op: f };
        self.fold_folder(basic_folder).await
    }
}


impl<'a, T: 'a> IntoIterator for Iter<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.slice.into_iter()
    }
}

#[async_trait]
impl<'a, T: 'a + Sync> AsyncIterator for Iter<'a, T> {
    type Item = &'a T;
    // si on est un Iter sur une tranche
    // on reprend le vieil algo: on coupe en blocs et chaque bloc est folde avec le folder qu'on a en argument
    async fn fold_folder<F:Folder<Self::Item>>(self, mut folder:F) -> F::B
    where
    Self: Send,
    F: Send
    {
        let mut slice = self.slice;
        loop {
            let block_size = slice.len().max(1_024);
            let (left, right) = slice.split_at(block_size);
            folder = folder.fold(left.into_iter());// = left.into_iter().fold(res, folder);
            if right.is_empty() {
                return folder.into_result();
            }

            slice = right;
            yield_now().await;
        }
    }

}


#[async_trait]
impl<R, I, O> AsyncIterator for Map<I, O> 
    where 
    I: AsyncIterator,
    O: Fn(I::Item) -> R,
    O: Send,
    I: Send {
    type Item = R;

    // pour le map: si on est Map<Iter>
    // on prend l'op du map et le Folder en argument et on les met dans un MapFolder qui implement le trait Folder
    // on appelle inner_iterator.fold_folder(mapfolder)
    async fn fold_folder<F:Folder<Self::Item>>(self, folder:F) -> F::B
    where
    Self: Send,
    F: Send
    {
        let map_folder = MapFolder { folder, op:self.op  };
        self.base.fold_folder(map_folder).await
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
