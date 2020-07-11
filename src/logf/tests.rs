use super::mock::{DescribeLogStreamsResponse, GeloLogEventsResponse};
use crate::tailf;
use regex::Regex;
use rusoto_core::Region;
use rusoto_logs::CloudWatchLogsClient;
use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher, MultipleMockRequestDispatcher};
use std::env;

#[tokio::test]
async fn test_stream_tailf() {
  env::set_var("TZ", "Asia/Tokyo");
  let mut buf = vec![];

  let client = CloudWatchLogsClient::new_with(
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse::new(
      vec![("hello", 0), ("world", 0)],
      None,
    )),
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
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse::new(
      vec![("hello", 0), ("world", 0)],
      None,
    )),
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
    MockRequestDispatcher::default().with_json_body(DescribeLogStreamsResponse::new(
      vec![("stream1", Some(1)), ("stream2", Some(2))],
      None,
    )),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse::new(
      vec![("event1", 3), ("event2", 3)],
      None,
    )),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse::new(
      vec![("event3", 3), ("event4", 3)],
      None,
    )),
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
    MockRequestDispatcher::default().with_json_body(DescribeLogStreamsResponse::new(
      vec![("stream1", Some(1)), ("stream2", Some(2))],
      None,
    )),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse::new(
      vec![("event1", 3), ("event2", 3)],
      None,
    )),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse::new(
      vec![("event3", 3), ("event4", 3)],
      None,
    )),
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

#[tokio::test]
async fn test_group_tailf_with_stream_filter() {
  env::set_var("TZ", "Asia/Tokyo");
  let mut buf = vec![];

  let responses = vec![
    MockRequestDispatcher::default().with_json_body(DescribeLogStreamsResponse::new(
      vec![("stream1", Some(1)), ("stream2", Some(2))],
      None,
    )),
    MockRequestDispatcher::default().with_json_body(GeloLogEventsResponse::new(
      vec![("event1", 3), ("event2", 3)],
      None,
    )),
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
    Some(Regex::new("^stream1$").unwrap()),
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
stream1\t1970-01-01T09:00:00.003+09:00\tevent2\n"
  );
}
