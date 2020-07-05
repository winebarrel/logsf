extern crate chrono;
extern crate rusoto_core;
extern crate rusoto_logs;

use chrono::TimeZone;
use regex::Regex;
use rusoto_core::Region;
use rusoto_logs::{
  CloudWatchLogs, CloudWatchLogsClient, DescribeLogStreamsRequest, GetLogEventsRequest,
  OutputLogEvent,
};
use std::{error, thread, time, vec::Vec};

async fn print_log_events(
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

async fn get_log_stream_names(
  client: &CloudWatchLogsClient,
  log_group_name: &str,
  filter: &Option<Regex>,
  last_timestamp: i64,
) -> Result<Vec<String>, Box<dyn error::Error>> {
  let mut next_token = None;
  let mut log_stream_names = vec![];

  'outer: loop {
    let req = DescribeLogStreamsRequest {
      log_group_name: log_group_name.to_string(),
      log_stream_name_prefix: None,
      descending: Some(true),
      order_by: Some("LastEventTime".to_string()),
      limit: None,
      next_token: next_token,
    };

    let res = client.describe_log_streams(req).await?;

    if res.log_streams.is_none() {
      return Ok(log_stream_names);
    }

    for log_stream in res.log_streams.unwrap() {
      if let Some(last_ingestion_time) = log_stream.last_ingestion_time {
        if last_ingestion_time < last_timestamp {
          break 'outer;
        }
      } else {
        continue;
      }

      let log_stream_name = log_stream.log_stream_name.unwrap();

      if let Some(re) = filter {
        if !re.is_match(&log_stream_name) {
          continue;
        }
      }

      log_stream_names.push(log_stream_name.to_string());
    }

    next_token = res.next_token;

    if next_token.is_none() {
      break;
    }
  }

  Ok(log_stream_names)
}

async fn print_group_log_events(
  client: &CloudWatchLogsClient,
  log_group_name: &str,
  filter: Option<Regex>,
  start_time: Option<i64>,
  verbose: bool,
) -> Result<(), Box<dyn error::Error>> {
  let mut last_timestamp = if let Some(ts) = start_time {
    ts
  } else {
    chrono::Utc::now().timestamp_millis()
  };

  loop {
    let log_stream_names =
      get_log_stream_names(&client, log_group_name, &filter, last_timestamp).await?;

    let mut stream_log_events: Vec<(String, OutputLogEvent)> = vec![];

    for log_stream_name in log_stream_names {
      let req = GetLogEventsRequest {
        log_group_name: log_group_name.to_string(),
        log_stream_name: log_stream_name.clone(),
        start_time: Some(last_timestamp),
        end_time: None,
        limit: None,
        start_from_head: Some(true),
        next_token: None,
      };

      let res = client.get_log_events(req).await?;

      if let Some(events) = res.events {
        for event in events {
          if event.timestamp.unwrap() > last_timestamp {
            stream_log_events.push((log_stream_name.clone(), event));
          }
        }
      }
    }

    stream_log_events.sort_by(|a, b| a.1.timestamp.partial_cmp(&b.1.timestamp).unwrap());

    for (log_stream_name, event) in stream_log_events {
      let message = event.message.unwrap();
      let timestamp = event.timestamp.unwrap();

      if verbose {
        let ts = chrono::Local.timestamp_millis(timestamp);
        println!("{}\t{}\t{}", log_stream_name, ts.to_rfc3339(), message);
      } else {
        println!("{}", message);
      }

      last_timestamp = timestamp;
    }

    thread::sleep(time::Duration::from_secs(1));
  }
}

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
