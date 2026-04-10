use std::{pin::Pin, sync::Arc};

use crate::errors::RTErrors;

pub type RTAsyncArcFn<T, O> =
    fn(Arc<T>) -> Pin<Box<dyn Future<Output = Result<O, RTErrors>> + Send>>;
pub type RTAsyncRefFn<T, O> =
    for<'a> fn(&'a T) -> Pin<Box<dyn Future<Output = Result<O, RTErrors>> + Send + 'a>>;
pub type RTAsyncMutRefFn<T, O> =
    for<'a> fn(&'a mut T) -> Pin<Box<dyn Future<Output = Result<O, RTErrors>> + Send + 'a>>;
