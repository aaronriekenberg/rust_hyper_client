extern crate chrono;
extern crate fern;
extern crate futures;
extern crate hyper;
#[macro_use] extern crate log;
extern crate tokio_core;
extern crate tokio_timer;

use chrono::prelude::Local;

use futures::{Future, Stream};

use hyper::Client;

use std::io;
use std::thread;
use std::sync::Arc;
use std::time::Duration;

use tokio_core::reactor::Core;

use tokio_timer::Timer;

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

  let handle = Arc::new(core.handle());

  let client = Client::new(&handle);

  let timer = Timer::default();

  let duration = Duration::from_millis(500);

  let wakeups = timer.interval(duration);

  let handle_clone = Arc::clone(&handle);

  let timer_task = wakeups.for_each(move |_| {
    info!("in timer_task");

    let uri = "http://raspberrypi:8081".parse().expect("unvalid uri");

    info!("uri = {}", uri);

    handle_clone.spawn(client.get(uri).and_then(move |res| {
      let status = res.status();
      res.body().concat2().and_then(move |body| {
        info!("got response status {} body length {}",
              status, String::from_utf8_lossy(&body).len());
        Ok(())
      })
    }).map_err(|e| { error!("get error {}", e); () }));

    Ok(())

  }).map_err(|e| { error!("timer error {}", e); () });

  handle.spawn(timer_task);

  info!("call core.run");

  loop {
    core.turn(None);
  }
}
