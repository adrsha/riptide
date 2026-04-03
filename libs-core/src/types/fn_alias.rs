use std::{pin::Pin, sync::Arc};

use crate::errors::RTErrors;

pub type RTAsyncArcFn<T, I, O> =
    fn(Arc<T>, I) -> Pin<Box<dyn Future<Output = Result<O, RTErrors>> + Send>>;
pub type RTAsyncRefFn<T, I, O> =
    for<'a> fn(&'a T, I) -> Pin<Box<dyn Future<Output = Result<O, RTErrors>> + Send + 'a>>;
