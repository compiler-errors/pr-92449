use std::future::Future;

trait Client {
    type Connecting<'a>: Future + Send
    where
        Self: 'a;

    fn connect(&'_ self) -> Self::Connecting<'_>;
}

fn call_connect<C>(c: &'_ C) -> impl '_ + Future + Send  // removing +Send will compile
where
    C: Client + Send + Sync,
{
    // c.connect().inspect(|_| println!("this compiles"))
    async move {
        println!("this does not compile");
        c.connect().await
    }
}

struct ClientWrapper<T>(T);

impl<T> Client for ClientWrapper<T>
where
    T: Client + Send + Sync,
{
    type Connecting<'a>
    where
        T: 'a,
    = impl Future + Send; // removing +Send here & in the trait will compile

    fn connect(&'_ self) -> Self::Connecting<'_> {
        // self.0.connect()  // <- this compiles
        // self.0.connect().inspect(|_| println!("this compiles"))
        async move {
            println!("this does not compile");
            self.0.connect().await
        }
    }
}