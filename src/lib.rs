use tokio::task::yield_now;
use async_trait::async_trait;

/// A simple iterator over a slice.
pub struct Iter<'a, T:'a> {
    /// The underlying slice being iterated over.
    pub slice: &'a [T]
}

impl<'a, T: 'a> IntoIterator for Iter<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    /// Converts the iterator into a standard slice iterator.
    fn into_iter(self) -> Self::IntoIter {
        self.slice.into_iter()
    }
}

/// An asynchronous iterator over a slice, capable of asynchronous folding.
///
/// This iterator adapts a slice of type `&'a [T]` and allows asynchronous folding using a provided folder.
///
/// # Type Parameters
/// - `'a`: The lifetime of the slice.
/// - `T`: The type of items in the slice.
#[async_trait]
impl<'a, T: 'a + Sync> AsyncIterator for Iter<'a, T> {
    /// The type of items yielded by the iterator.
    type Item = &'a T;

    /// Asynchronously folds the iterator using the provided folder.
    ///
    /// # Parameters
    /// - `folder`: The folder used for asynchronous folding.
    ///
    /// # Returns
    /// The result of folding, represented by the type `F::B`.
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

/// An asynchronous iterator trait.
#[async_trait]
pub trait AsyncIterator: Sized {
    /// The type of items yielded by the iterator.
    type Item;

    /// Asynchronously folds the iterator using the provided folder.
    async fn fold_folder<F:Folder<Self::Item>>(self, folder:F) -> F::B
    where
    Self: Send,
    F: Send;

    /// Asynchronously folds the iterator using the base folder with an initial value and operation.
    async fn async_fold<B, F>(mut self, init: B, f: F) -> B
    where
        Self::Item : Send,
        B: Send,
        F: Fn(B, Self::Item) -> B + Send + Copy,
    {
        let basic_folder = BasicFolder { init, op: f };
        self.fold_folder(basic_folder).await
    }

    /// Maps each item of the iterator using the provided operation.
    fn map<O, R>(self, op:O) -> Map<Self, O>
    where O: FnMut(Self::Item) -> R, 
    {
        Map {
            base: self,
            op,
        }
    }

    /// Filters items of the iterator based on the provided predicate.
    fn filter<P>(self, pred:P) -> Filter<Self, P>
    where P: FnMut(&Self::Item) -> bool, 
    {
        Filter {
            base: self,
            pred,
        }
    }

    /// Collects the items of the iterator into a vector asynchronously.
    async fn collect_vec(self) -> Vec<Self::Item> 
    where
        Self::Item : Send
    {
        self.async_fold(Vec::new(), |mut v, e| {v.push(e); v}).await
    }

}

/// A trait representing a folder for an iterator.
pub trait Folder<Item> {
    /// The type of the result after folding.
    type B;

    /// Folds the iterator using the provided folder.
    fn fold<I:Iterator<Item=Item>>(self, i: I) -> Self;

    /// Converts the folder into its final result.
    fn into_result(self) -> Self::B;
}

/// The base folder implementation.
pub struct BasicFolder<B, O> {
    /// The initial value for folding.
    init: B,
    /// The folding operation.
    op: O,
}

/// The base folder implementation that performs a simple fold operation.
///
/// This folder accumulates values of type `Item` into a result of type `B` using a binary operation.
/// It is a simple building block for more complex folding operations.
///
/// # Type Parameters
/// - `Item`: The type of items being folded.
/// - `B`: The type of the result produced by the fold operation.
/// - `O`: The type of the folding operation.
impl<Item, B, O> Folder<Item> for BasicFolder<B, O> where 
O: Fn(B, Item) -> B {
    /// The type of result produced by the fold operation.
    type B = B;

    /// Folds the iterator using the provided binary operation.
    ///
    /// # Parameters
    /// - `i`: The iterator of items to be folded.
    ///
    /// # Returns
    /// A new instance of `BasicFolder` with the updated initial value.
    fn fold<I: Iterator<Item=Item>>(mut self, i:I) -> Self {
        let new_b = i.fold(self.init, &self.op);
        self.init = new_b;
        self
    }
    
    fn into_result(self) -> Self::B {
        self.init
    }
}

/// A map iterator that transforms each item using the provided operation.
pub struct Map<I, O> {
    /// The base iterator.
    base: I,
    /// The mapping operation.
    op: O,
}

/// An asynchronous iterator that maps items using a provided operation.
///
/// This iterator adapts another asynchronous iterator `I` by applying a mapping operation `O` to its items.
/// It produces items of type `R` as a result.
///
/// # Type Parameters
/// - `R`: The type of items produced by the mapping operation.
/// - `I`: The inner asynchronous iterator type that this `Map` iterator adapts.
/// - `O`: The type of the mapping operation.
#[async_trait]
impl<R, I, O> AsyncIterator for Map<I, O> 
    where 
    I: AsyncIterator,
    O: Fn(I::Item) -> R,
    O: Send,
    I: Send 
{
    /// The type of items yielded by the iterator.
    type Item = R;

    /// Asynchronously folds the iterator using the provided folder.
    ///
    /// # Parameters
    /// - `folder`: The folder used for folding the items.
    ///
    /// # Returns
    /// The result of folding, represented by the type `F::B`.
    async fn fold_folder<F:Folder<Self::Item>>(self, folder:F) -> F::B
    where
    Self: Send,
    F: Send
    {
        let map_folder = MapFolder { folder, op:self.op  };
        self.base.fold_folder(map_folder).await
    }
}

/// A folder for the map iterator.
pub struct MapFolder<F, O> {
    /// The inner folder.
    folder: F,
    /// The mapping operation.
    op: O,
}

/// A folder implementation that maps items using a provided operation.
///
/// This folder adapts another folder `F` by applying a mapping operation `O` to its items.
/// It preserves the original folder's type `F::B` as the result type.
///
/// # Type Parameters
/// - `Item`: The type of items being folded.
/// - `R`: The type of items produced by the mapping operation.
/// - `F`: The inner folder type that this `MapFolder` adapts.
/// - `O`: The type of the mapping operation.
impl<Item, R, F, O> Folder<Item> for MapFolder<F, O> where
F: Folder<R>,
O: Fn(Item) -> R {
    /// The type of result produced by the adapted folder.
    type B = F::B;

    /// Folds the iterator by applying the mapping operation to the items.
    ///
    /// # Parameters
    /// - `i`: The iterator of items to be folded.
    ///
    /// # Returns
    /// A new instance of `MapFolder` with the updated inner folder.
    fn fold<I: Iterator<Item=Item>>(mut self, i:I) -> Self {
        let (base, op) = (self.folder, self.op);
        let new_base_folder = base.fold(i.map(&op));
        self.folder = new_base_folder;
        self.op = op;
        self
    }

    /// Converts the `MapFolder` into its final result.
    ///
    /// # Returns
    /// The result produced by the adapted inner folder.
    fn into_result(self) -> Self::B {
        self.folder.into_result()
    }
}

/// A filter iterator that selects items based on the provided predicate.
pub struct Filter<I, P> {
    /// The base iterator.
    base: I,
    /// The filtering predicate.
    pred: P,
}

/// An asynchronous iterator that filters items based on a predicate.
///
/// This iterator adapts another asynchronous iterator `I` by applying a filtering predicate `P` to its items.
/// It preserves the original iterator's item type `I::Item`.
///
/// # Type Parameters
/// - `I`: The inner asynchronous iterator type that this `Filter` iterator adapts.
/// - `P`: The type of the filtering predicate.
#[async_trait]
impl<I, P> AsyncIterator for Filter<I, P> 
    where 
    I: AsyncIterator,
    P: FnMut(&I::Item) -> bool,
    P: Send,
    I: Send 
{
    /// The type of items yielded by the iterator.
    type Item = I::Item;

    /// Asynchronously folds the iterator using the provided folder.
    ///
    /// # Parameters
    /// - `folder`: The folder used for folding the items.
    ///
    /// # Returns
    /// The result of folding, represented by the type `F::B`.
    async fn fold_folder<F:Folder<Self::Item>>(self, folder:F) -> F::B
    where
    Self: Send,
    F: Send
    {
        let filter_folder = FilterFolder { folder, pred:self.pred };
        self.base.fold_folder(filter_folder).await
    }
}

/// A folder for the filter iterator.
pub struct FilterFolder<F, P> {
    /// The inner folder.
    folder: F,
    /// The filtering predicate.
    pred: P,
}

/// A folder implementation that filters items based on a predicate.
///
/// This folder adapts another folder `F` by applying a filtering predicate `P` to its items.
/// It preserves the original folder's type `F::B` as the result type.
///
/// # Type Parameters
/// - `Item`: The type of items being folded.
/// - `F`: The inner folder type that this `FilterFolder` adapts.
/// - `P`: The type of the filtering predicate.
impl<Item, F, P> Folder<Item> for FilterFolder<F, P> where
F: Folder<Item>,
P: FnMut(&Item) -> bool {
    /// The type of result produced by the adapted folder.
    type B = F::B;

    /// Folds the iterator by applying the filtering predicate to the items.
    ///
    /// # Parameters
    /// - `i`: The iterator of items to be folded.
    ///
    /// # Returns
    /// A new instance of `FilterFolder` with the updated inner folder.
    fn fold<I: Iterator<Item=Item>>(mut self, i:I) -> Self {
        let (base, mut pred) = (self.folder, self.pred);
        let new_base_folder = base.fold(i.filter(&mut pred));
        self.folder = new_base_folder;
        self.pred = pred;
        self
    }

    /// Converts the `FilterFolder` into its final result.
    ///
    /// # Returns
    /// The result produced by the adapted inner folder.
    fn into_result(self) -> Self::B {
        self.folder.into_result()
    }
}
