// Currently fails because our param-env has two predicates:
//     <T as Foo<'a>>
// and
//     <T as Foo<'0>>
// and for some reason, that causes us to fail normalizing
//     <T as Foo<'0>>::Future
// which then causes us to error.

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
