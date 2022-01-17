// This passes with the PR


use core::future;
use futures::stream::{self, StreamExt};
use parking_lot::RwLock;
use std::borrow::Cow;
use std::sync::Arc;

struct Bar {}

impl Bar {
    async fn get_foo<'a>(&self) -> Vec<Foo<'a>> {
        let results: Vec<Result<String, ()>> =
            vec![Ok("shoutout to `impl Fox for Salix` for this vector".to_string()); 10];
        let foo: Vec<Foo<'a>> = stream::iter(results)
            .filter_map(|r| async { r.ok() })
            .filter_map(|m| async move { Some(Foo::new(m)) })
            .filter(|m| future::ready(test_foo(m.as_str()))) // Code compiles without this line
            .collect()
            .await;
        foo
    }

    async fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

struct Foo<'a> {
    raw: Cow<'a, str>,
}

impl<'a> Foo<'a> {
    fn as_str(&self) -> &str {
        &self.raw
    }

    fn new<T>(raw: T) -> Foo<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Foo { raw: raw.into() }
    }
}

struct FooBar<'a> {
    bar: Arc<Bar>,
    foo: RwLock<Vec<Foo<'a>>>,
}

impl<'a> FooBar<'a> {
    async fn get_foo(&self) -> Vec<Foo<'a>> {
        let foo: Vec<Foo<'a>> = self.bar.get_foo().await;
        foo
    }

    async fn new(bar: Arc<Bar>) -> Arc<FooBar<'a>> {
        let foobar = FooBar {
            bar: bar.clone(),
            foo: RwLock::new(Vec::new()),
        };
        Arc::new(foobar)
    }

    async fn run(&self) {
        let new_foo: Vec<Foo<'a>> = self.get_foo().await;
        {
            let mut foo = self.foo.write();
            *foo = new_foo;
        }
    }
}

async fn main() {
    let bar = Bar::new().await;
    let foobar = FooBar::new(bar).await;
    let foobar_runner = tokio::spawn(async move {
        foobar.run().await;
    })
    .await;
}

fn test_foo(foo: &str) -> bool {
    println!("{}", foo);
    true
}