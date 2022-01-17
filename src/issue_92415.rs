use std::any::Any;
use std::future::Future;

trait Foo<'a>: Sized {
    type Error;
    fn foo(x: &'a str) -> Result<Self, Self::Error>;
}

impl<'a> Foo<'a> for &'a str {
    type Error = ();

    fn foo(x: &'a str) -> Result<Self, Self::Error> {
        Ok(x)
    }
}

async fn get_foo<'a, T>(x: &'a str) -> Result<T, <T as Foo<'a>>::Error>
where
    T: Foo<'a>,
{
    Foo::foo(x)
}

fn bar<'a>(x: &'a str) -> Box<dyn Future<Output = Result<&'a str, ()>> + Send + 'a> {
    Box::new(async move { get_foo(x).await })
}