extern crate futures;
extern crate tokio_core;
extern crate tokio_threadpool;
extern crate hyper;

use std::sync::Arc;
use hyper::server::{Service, Http};
use hyper::{Request, Response, Error};
use futures::{Future, Stream};
use futures::sync::oneshot;
use std::time::Instant;
use std::thread;
use std::time::Duration;

type ServiceFuture = Box<Future<Item = self::Response, Error = hyper::error::Error>>;

use tokio_threadpool::{ThreadPool, Builder as ThreadPoolBuilder};

fn duration_to_millis(duration: Duration) -> f64 {
    let nanoseconds = u64::from(duration.subsec_nanos());
    let ms = duration
        .as_secs()
        .saturating_mul(1000)
        .saturating_add(nanoseconds / (1000 * 1000));
    ms as f64
}

struct Server {
    pool: Arc<ThreadPool>,
}


impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = ServiceFuture;

    fn call(&self, req: Request) -> Self::Future {
        let pool = Arc::clone(&self.pool);
        let future = req
            .body()
            .concat2()
            .map(move |body| (body, Instant::now()))
            .and_then(move |(body, timestamp)| {
                oneshot::spawn_fn(move || {
                    println!("Future waited {:?}", duration_to_millis(timestamp.elapsed()));
                    thread::sleep(Duration::from_millis(200));
                    Ok(Response::new())
                }, pool.sender())
            });
        Box::new(future)
    }
}

fn main() {
    let pool = Arc::new(
        ThreadPoolBuilder::new()
            .pool_size(4)
            .name_prefix("search-thread-pool-")
            .build()
    );
    let service_addr = "127.0.0.1:1488".parse().unwrap();
    let service = Arc::new(Server { pool });
    let srv = Http::new().bind(&service_addr, move || Ok(Arc::clone(&service))).unwrap();
    srv.run();
}
