extern crate rusoto_core;
extern crate rusoto_logs;

mod group;
mod stream;

use group::print_group_log_events;
use regex::Regex;
use rusoto_core::Region;
use rusoto_logs::CloudWatchLogsClient;
use std::error;
use stream::print_log_events;

pub async fn tailf(
  log_group_name: &str,
  log_stream_name: Option<String>,
  filter: Option<Regex>,
  start_time: Option<i64>,
  verbose: bool,
) -> Result<(), Box<dyn error::Error>> {
  let client = CloudWatchLogsClient::new(Region::default());

  if let Some(stream) = log_stream_name {
    print_log_events(&client, &log_group_name, &stream, start_time, verbose).await
  } else {
    print_group_log_events(&client, &log_group_name, filter, start_time, verbose).await
  }
}
