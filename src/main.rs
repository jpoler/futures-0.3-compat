#![feature(async_await, await_macro, futures_api, tokio_compat)]

extern crate futures;
extern crate hyper;

use futures::executor::block_on;
use futures::future::{self, FutureExt};
use futures::compat::Compat;

async fn foo(a: &str) {
    await!(bar(a));
}

async fn bar(a: &str) {
    println!("a: {}", a);
}

fn main() {
/*   let fut = async {
    let client = hyper::Client::new();
    let uri = "https://www.google.com".parse::<hyper::Uri>().unwrap();
    let resp = await!(client.get(uri).compat());
    println!("resp: {}", resp);
  };
  block_on(fut) */
}