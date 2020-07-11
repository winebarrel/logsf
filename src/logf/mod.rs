extern crate rusoto_core;
extern crate rusoto_logs;

mod group;
mod stream;

#[cfg(test)]
mod tests;

use group::print_group_log_events;
use regex::Regex;
use rusoto_logs::CloudWatchLogs;
use std::error;
use std::io;
use stream::print_log_events;

pub async fn tailf<C, W>(
  client: &C,
  log_group_name: &str,
  log_stream_name: Option<String>,
  filter: Option<Regex>,
  start_time: Option<i64>,
  verbose: bool,
  out: &mut W,
  wait: Option<u64>,
) -> Result<(), Box<dyn error::Error>>
where
  C: CloudWatchLogs,
  W: io::Write,
{
  if let Some(stream) = log_stream_name {
    print_log_events(
      client,
      &log_group_name,
      &stream,
      start_time,
      verbose,
      out,
      wait,
    )
    .await
  } else {
    print_group_log_events(
      client,
      &log_group_name,
      filter,
      start_time,
      verbose,
      out,
      wait,
    )
    .await
  }
}
