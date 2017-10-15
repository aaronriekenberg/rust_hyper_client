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

fn initialize_logging() -> Result<(), fern::InitError> {
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

  let handle = core.handle();

  let client = Client::new(&handle);

  (0..20).for_each(|i| {
    let uri = "http://raspberrypi:8081".parse().expect("unvalid uri");

    info!("i = {} uri = {}", i, uri);

    handle.spawn(client.get(uri).and_then(move |res| {
      let status = res.status();
      res.body().concat2().and_then(move |body| {
        info!("i = {} got response status {} body length {}",
              i, status, String::from_utf8_lossy(&body).len());
        Ok(())
      })
    }).map(|_| ()).map_err(|_| ()));
  });

  info!("call core.run");

  loop {
    core.turn(None);
  }
}
