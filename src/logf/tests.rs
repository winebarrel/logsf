extern crate rusoto_mock;
extern crate serde_json;

use super::tailf;
use rusoto_core::Region;
use rusoto_logs::CloudWatchLogsClient;
use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher, MultipleMockRequestDispatcher};
use serde::Serialize;
use std::env;

#[derive(Debug, Serialize)]
struct LogStream {
  #[serde(rename = "logStreamName")]
  log_stream_name: String,
  #[serde(rename = "lastIngestionTime", skip_serializing_if = "Option::is_none")]
  last_ingestion_time: Option<i64>,
}

#[derive(Debug, Serialize)]
struct DescribeLogStreamsResponse {
  #[serde(rename = "logStreams")]
  log_streams: Vec<LogStream>,
  #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
  next_token: Option<String>,
}

#[derive(Debug, Serialize)]
struct LogEvent {
  message: String,
  timestamp: i64,
}

#[derive(Debug, Serialize)]
struct GeloLogEventsResponse {
  events: Vec<LogEvent>,
  #[serde(rename = "nextForwardToken", skip_serializing_if = "Option::is_none")]
  next_forward_token: Option<String>,
}

#[tokio::test]
async fn test_stream_tailf() {
  env::set_var("TZ", "Asia/Tokyo");
  let mut buf = vec![];

  let client = CloudWatchLogsClient::new_with(
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse {
      events: vec![
        LogEvent {
          message: "hello".to_string(),
          timestamp: 0,
        },
        LogEvent {
          message: "world".to_string(),
          timestamp: 0,
        },
      ],
      next_forward_token: None,
    }),
    MockCredentialsProvider,
    Region::UsEast1,
  );

  tailf(
    &client,
    "log_group_name",
    Some("log_stream_name".to_string()),
    None,
    None,
    false,
    &mut buf,
    None,
  )
  .await
  .unwrap();

  assert_eq!(String::from_utf8(buf).unwrap(), "hello\nworld\n");
}

#[tokio::test]
async fn test_stream_tailf_verbose() {
  env::set_var("TZ", "Asia/Tokyo");
  let mut buf = vec![];

  let client = CloudWatchLogsClient::new_with(
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse {
      events: vec![
        LogEvent {
          message: "hello".to_string(),
          timestamp: 0,
        },
        LogEvent {
          message: "world".to_string(),
          timestamp: 0,
        },
      ],
      next_forward_token: None,
    }),
    MockCredentialsProvider,
    Region::UsEast1,
  );

  tailf(
    &client,
    "log_group_name",
    Some("log_stream_name".to_string()),
    None,
    None,
    true,
    &mut buf,
    None,
  )
  .await
  .unwrap();

  assert_eq!(
    String::from_utf8(buf).unwrap(),
    "1970-01-01T09:00:00+09:00\thello
1970-01-01T09:00:00+09:00\tworld\n"
  );
}

#[tokio::test]
async fn test_group_tailf() {
  env::set_var("TZ", "Asia/Tokyo");
  let mut buf = vec![];

  let responses = vec![
    MockRequestDispatcher::default().with_json_body(DescribeLogStreamsResponse {
      log_streams: vec![
        LogStream {
          log_stream_name: "stream1".to_string(),
          last_ingestion_time: Some(1),
        },
        LogStream {
          log_stream_name: "stream2".to_string(),
          last_ingestion_time: Some(2),
        },
      ],
      next_token: None,
    }),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse {
      events: vec![
        LogEvent {
          message: "event1".to_string(),
          timestamp: 3,
        },
        LogEvent {
          message: "event2".to_string(),
          timestamp: 3,
        },
      ],
      next_forward_token: None,
    }),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse {
      events: vec![
        LogEvent {
          message: "event3".to_string(),
          timestamp: 3,
        },
        LogEvent {
          message: "event4".to_string(),
          timestamp: 3,
        },
      ],
      next_forward_token: None,
    }),
  ];

  let client = CloudWatchLogsClient::new_with(
    MultipleMockRequestDispatcher::new(responses),
    MockCredentialsProvider,
    Region::UsEast1,
  );

  tailf(
    &client,
    "log_group_name",
    None,
    None,
    Some(0),
    false,
    &mut buf,
    None,
  )
  .await
  .unwrap();

  assert_eq!(
    String::from_utf8(buf).unwrap(),
    "event1\nevent2\nevent3\nevent4\n"
  );
}

#[tokio::test]
async fn test_group_tailf_verbose() {
  env::set_var("TZ", "Asia/Tokyo");
  let mut buf = vec![];

  let responses = vec![
    MockRequestDispatcher::default().with_json_body(DescribeLogStreamsResponse {
      log_streams: vec![
        LogStream {
          log_stream_name: "stream1".to_string(),
          last_ingestion_time: Some(1),
        },
        LogStream {
          log_stream_name: "stream2".to_string(),
          last_ingestion_time: Some(2),
        },
      ],
      next_token: None,
    }),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse {
      events: vec![
        LogEvent {
          message: "event1".to_string(),
          timestamp: 3,
        },
        LogEvent {
          message: "event2".to_string(),
          timestamp: 3,
        },
      ],
      next_forward_token: None,
    }),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse {
      events: vec![
        LogEvent {
          message: "event3".to_string(),
          timestamp: 3,
        },
        LogEvent {
          message: "event4".to_string(),
          timestamp: 3,
        },
      ],
      next_forward_token: None,
    }),
  ];

  let client = CloudWatchLogsClient::new_with(
    MultipleMockRequestDispatcher::new(responses),
    MockCredentialsProvider,
    Region::UsEast1,
  );

  tailf(
    &client,
    "log_group_name",
    None,
    None,
    Some(0),
    true,
    &mut buf,
    None,
  )
  .await
  .unwrap();

  assert_eq!(
    String::from_utf8(buf).unwrap(),
    "stream1\t1970-01-01T09:00:00.003+09:00\tevent1
stream1\t1970-01-01T09:00:00.003+09:00\tevent2
stream2\t1970-01-01T09:00:00.003+09:00\tevent3
stream2\t1970-01-01T09:00:00.003+09:00\tevent4\n"
  );
}
