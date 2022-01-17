use std::{future::Future, marker::PhantomData, pin::Pin};

pub trait Foo<'a> {
    type Future: Future<Output = ()>;

    fn foo() -> Self::Future;
}

struct MyType<T>(PhantomData<T>);

impl<'a, T> Foo<'a> for MyType<T>
where
    T: Foo<'a>,
    T::Future: Send,
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn foo() -> Self::Future {
        Box::pin(async move { T::foo().await })
    }
}
