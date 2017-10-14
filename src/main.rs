extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
  let mut core = Core::new().expect("error creating core");
  let client = Client::new(&core.handle());

  let uri = "http://raspberrypi:8081".parse().expect("unvalid uri");
  let work = client.get(uri).and_then(|res| {
    println!("Response: {:#?}", res);

    res.body().for_each(|chunk| {
      io::stdout()
          .write_all(&chunk)
          .map_err(From::from)
    })
  });

  let result = core.run(work);
  match result {
    Ok(_) => println!("\nsuccess"),
    Err(e) => println!("\nerror: {}", e)
  };
}
