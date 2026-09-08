use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id<T> {
    pub id: String, // TODO(perf): Use small strings? #3
    _marker: PhantomData<fn() -> T>,
}
