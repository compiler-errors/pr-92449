// This fails because we erase the 'static region in the 
// `-> Box<dyn Any>`, which means that `callback` is no longer `Foo`.

use std::{any::Any, fmt, future::Future};

pub trait Foo {
    type Item;
}

impl<F, I> Foo for F
where
    Self: FnOnce() -> I,
    I: fmt::Debug,
{
    type Item = I;
}

async fn foo_item<F: Foo>(_: F) -> F::Item {
    unimplemented!()
}

pub fn foo() {
    let fut = async {
        let callback = || -> Box<dyn Any> { unimplemented!() };
        foo_item(callback).await;
    };

    let fut: &(dyn Future<Output = ()> + Send) = &fut as _;
}
