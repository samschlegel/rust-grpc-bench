use anymap::AnyMap;
use std::any::TypeId;
use std::boxed::Box;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type Params = AnyMap;
pub type Error = ();
pub struct Role(String);

pub trait Param: Default + Debug + Sized {
    fn type_id(&self) -> &'static TypeId;
}

pub trait Head {
    fn role(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> Vec<TypeId>;
}

pub trait Stackable<T>: Head {
    fn to_stack(self, next: Stack<T>) -> Stack<T>;
}

pub enum Stack<T> {
    Node(
        Arc<Box<dyn Head>>,
        Box<dyn FnOnce(&Params, Box<Stack<T>>) -> Stack<T>>,
        Box<Stack<T>>,
    ),
    Leaf(Arc<Box<dyn Head>>, T),
}

pub fn simple_node<H: Head + 'static, T, Mk: FnOnce(T) -> T + 'static>(head: H, mk: Mk, next: Stack<T>) -> Stack<T> {
    let h: Arc<Box<dyn Head>> = Arc::new(Box::from(head));
    let h2 = h.clone();
    let f = move |p: &Params, stk: Box<Stack<T>>| {
        Stack::Leaf(h2, mk(stk.make(p)))
    };
    Stack::Node(h, Box::from(f), Box::from(next))
}

pub fn leaf<H: Head + 'static, T>(head: H, )

impl<T> Stack<T> {
    fn make(self, params: &Params) -> T {
        match self {
            Stack::Node(_, mk, next) => mk(params, next).make(params),
            Stack::Leaf(_, t) => t,
        }
    }

    fn prepend<S: Stackable<T>>(self, stk: S) -> Stack<T> {
        stk.to_stack(self)
    }
}

pub trait Service<Req, Rep> {
    fn call(
        &mut self,
        req: Req,
    ) -> Pin<Box<dyn Future<Output = Result<Rep, Error>> + Send + Sync + 'static>>;
}

pub trait ClientConnection {}

pub trait ServiceFactory<Req, Rep> {
    fn call(
        &mut self,
        conn: Box<dyn ClientConnection>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Box<dyn Service<Req, Rep> + Send + Sync + 'static>, Error>>
                + Send
                + Sync
                + 'static,
        >,
    >;
}

pub struct NullClientConnection {}
impl ClientConnection for NullClientConnection {}

pub struct Router<Req, Rep> {
    inner: Box<dyn ServiceFactory<Req, Rep>>,
}

impl<Req, Rep> ServiceFactory<Req, Rep> for Router<Req, Rep> {
    fn call(
        &mut self,
        conn: Box<dyn ClientConnection>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Box<dyn Service<Req, Rep> + Send + Sync + 'static>, Error>>
                + Send
                + Sync
                + 'static,
        >,
    > {
        self.inner.call(conn)
    }
}
impl<Req, Rep> Head for Router<Req, Rep> {
    fn role(&self) -> String {
        "router".to_string()
    }
    fn description(&self) -> String {
        "Router".to_string()
    }

    fn parameters(&self) -> Vec<TypeId> {
        Vec::new()
    }
}

impl<Req, Rep> Stackable<Box<dyn ServiceFactory<Req, Rep>>> for Router<Req, Rep>
where
    Req: 'static,
    Rep: 'static,
{
    fn to_stack(self, next: Stack<Box<dyn ServiceFactory<Req, Rep>>>) -> Stack<Box<dyn ServiceFactory<Req, Rep>>> {
        let h = Arc::new(Box::from(self) as Box<dyn Head>);
        let h2 = h.clone();
        let mk = |p: &Params, next: Box<Stack<Box<dyn ServiceFactory<Req, Rep>>>>| {
            let inner = Box::from(next.make(p));
            Stack::Leaf(h, Box::from(Router { inner }) as Box<dyn ServiceFactory<Req, Rep>>)
        };
        Stack::Node(
            h2,
            Box::from(mk),
            Box::from(next),
        )
    }
}

// These are generated
struct MyGrpcReq {}
struct MyGrpcRep {}
struct MyGrpcService {}
impl Service<MyGrpcReq, MyGrpcRep> for MyGrpcService {
    fn call(
        &mut self,
        req: MyGrpcReq,
    ) -> Pin<Box<dyn Future<Output = Result<MyGrpcRep, Error>> + Send + Sync + 'static>> {
        Box::pin(futures_0_3::future::ok(MyGrpcRep {}))
    }
}

struct MyGrpcServiceFactory {}
type CallResult = Pin<
    Box<
        dyn Future<Output = Result<Box<dyn Service<MyGrpcReq, MyGrpcRep> + Send + Sync>, Error>>
            + Send
            + Sync
            + 'static,
    >,
>;
impl ServiceFactory<MyGrpcReq, MyGrpcRep> for MyGrpcServiceFactory {
    fn call(&mut self, conn: Box<dyn ClientConnection>) -> CallResult {
        Box::pin(futures_0_3::future::ready(Ok(
            Box::from(MyGrpcService {}) as Box<dyn Service<MyGrpcReq, MyGrpcRep> + Send + Sync>
        )))
    }
}
impl Head for MyGrpcServiceFactory {
    fn role(&self) -> String {
        "mygrpcservicefactory".to_string()
    }
    fn description(&self) -> String {
        "My gRPC Service Factory".to_string()
    }

    fn parameters(&self) -> Vec<TypeId> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build() {
        // let node = simple_node()
    }
}
