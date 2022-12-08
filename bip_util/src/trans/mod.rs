pub use self::locally_shuffled::LocallyShuffledIds;
pub use self::sequential::SequentialIds;

mod sequential;
mod locally_shuffled;

pub trait TransactionIds<T> {
    fn generate(&mut self) -> T;
}

