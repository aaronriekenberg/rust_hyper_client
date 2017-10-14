extern crate chrono;
extern crate fern;
extern crate futures;
extern crate hyper;
#[macro_use] extern crate log;
extern crate tokio_core;

use chrono::prelude::Local;

use futures::{Future, Stream};

use hyper::Client;

use std::io;
use std::thread;

use tokio_core::reactor::Core;

fn initialize_logging() -> Result<(), fern::InitError>{
  fern::Dispatch::new()
    .level(log::LogLevelFilter::Info)
    .format(|out, message, record| {
      out.finish(
        format_args!("{} [{}] {} {} - {}",
          Local::now().format("%Y-%m-%d %H:%M:%S%.3f %z"),
          thread::current().name().unwrap_or("UNKNOWN"),
          record.level(),
          record.target(),
          message
        )
      )
    })
    .chain(io::stdout())
    .apply()?;

  Ok(())
}

fn main() {
  initialize_logging().expect("failed to initialize logging");

  let mut core = Core::new().expect("error creating core");

  let client = Client::new(&core.handle());

  let uri = "http://raspberrypi:8081".parse().expect("unvalid uri");

  info!("uri = {}", uri);

  let work = client.get(uri).and_then(|res| res.body().concat2());

  info!("call core.run");

  match core.run(work) {
    Ok(body) => info!("success body_string = {}", String::from_utf8_lossy(&body)),
    Err(e) => info!("error: {}", e)
  };
}
