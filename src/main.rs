#![feature(async_await, await_macro, futures_api, compat, tokio_compat)]
#[macro_use]
extern crate futures;
extern crate futures01;
extern crate hyper;
extern crate hyper_tls;

use futures::{FutureExt, TryFutureExt};
use futures::compat::{Future01CompatExt};
use futures::compat::TokioDefaultExecutor;
use tokio;
use tokio_threadpool::blocking;
use futures01::{Future as Future01, Stream as Stream01};
use futures01::future::{lazy, poll_fn};
use std::thread;

use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::rt::Stream;

use std::time::{Duration, Instant};

struct Downloader;

impl Downloader {
	async fn download() {
		let timer = tokio::timer::Interval::new(Instant::now(), Duration::from_millis(10))
		.take(1000)
		.for_each(|_| {
			println!("tick: {:?}", thread::current().id());
			Ok(())
			})
		.map_err(|e| panic!("timer error: {}", e));

		let join_handle = spawn_with_handle!(timer.compat()).unwrap();

		let https = HttpsConnector::new(4).expect("TLS initialization failed");
		let client = Client::builder()
   		.build::<_, hyper::Body>(https);
   	let uri = "https://dist.ipfs.io/go-ipfs/v0.4.17/go-ipfs_v0.4.17_darwin-amd64.tar.gz".parse::<hyper::Uri>().unwrap();
   	let resp = await!(client.get(uri).compat()).unwrap();
		let body = resp.into_body().concat2().compat();
		let buf = await!(body).unwrap();
   	println!("len buf: {}", buf.len());


		let blocking_write = poll_fn(|| {
			blocking(|| {
				// std::fs::write("/tmp/testing", buf).unwrap();
				println!("blocking operation start: {:?}", thread::current().id());
				thread::sleep(Duration::from_millis(100));
				println!("blocking operation stop: {:?}", thread::current().id());
			}).map_err(|e| panic!("error writing file: {}", e))
		});
		await!(blocking_write.compat());
		await!(join_handle);
 	}
}

fn main() {
	let future_compat = Downloader::download()
		.boxed()
		.unit_error()
		.compat(TokioDefaultExecutor);

		tokio::run(future_compat);
}