extern crate chrono;
extern crate rusoto_logs;

use chrono::TimeZone;
use rusoto_logs::{CloudWatchLogs, CloudWatchLogsClient, GetLogEventsRequest};
use std::{error, thread, time};

pub async fn print_log_events(
  client: &CloudWatchLogsClient,
  log_group_name: &str,
  log_stream_name: &str,
  start_time: Option<i64>,
  verbose: bool,
) -> Result<(), Box<dyn error::Error>> {
  let mut next_token = None;
  let start_from_head = if start_time.is_some() {
    Some(true)
  } else {
    None
  };

  loop {
    let req = GetLogEventsRequest {
      log_group_name: log_group_name.to_string(),
      log_stream_name: log_stream_name.to_string(),
      start_time: start_time,
      end_time: None,
      limit: None,
      start_from_head: start_from_head,
      next_token: next_token.clone(),
    };

    let res = client.get_log_events(req).await?;

    if let Some(events) = res.events {
      for event in events {
        let message = event.message.unwrap();

        if verbose {
          let ts = chrono::Local.timestamp_millis(event.timestamp.unwrap());
          println!("{}\t{}", ts.to_rfc3339(), message);
        } else {
          println!("{}", message);
        }
      }
    }

    next_token = res.next_forward_token;

    if next_token.is_none() {
      break;
    }

    thread::sleep(time::Duration::from_secs(1));
  }

  Ok(())
}
