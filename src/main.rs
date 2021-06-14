extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

fn main() {
    // Core is the Tokio event loop used for making a non-blocking request
    let mut core = Core::new().unwrap();

    let client = Client::new();

    let url : Uri = "http://pv.sohu.com/cityjson?ie=utf-8".parse().unwrap();
    // assert_eq!(url.query(), Some("foo=bar"));

    let request_result = core.run(client
        .get(url)
        .map(|res| {
            println!("Response: {:?}", res.body());
        })
        .map_err(|err| {
            println!("Error: {}", err);
       })
    );

}