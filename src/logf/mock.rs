use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LogStream {
  #[serde(rename = "logStreamName")]
  pub log_stream_name: String,
  #[serde(rename = "lastIngestionTime", skip_serializing_if = "Option::is_none")]
  pub last_ingestion_time: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DescribeLogStreamsResponse {
  #[serde(rename = "logStreams")]
  pub log_streams: Vec<LogStream>,
  #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
  pub next_token: Option<String>,
}

impl DescribeLogStreamsResponse {
  pub fn new(
    log_streams: &[(&str, Option<i64>)],
    next_token: Option<String>,
  ) -> DescribeLogStreamsResponse {
    DescribeLogStreamsResponse {
      log_streams: log_streams
        .iter()
        .map(|s| LogStream {
          log_stream_name: s.0.to_string(),
          last_ingestion_time: s.1,
        })
        .collect(),
      next_token: next_token,
    }
  }
}

#[derive(Debug, Serialize)]
pub struct LogEvent {
  pub message: String,
  pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct GeloLogEventsResponse {
  pub events: Vec<LogEvent>,
  #[serde(rename = "nextForwardToken", skip_serializing_if = "Option::is_none")]
  pub next_forward_token: Option<String>,
}

impl GeloLogEventsResponse {
  pub fn new(events: &[(&str, i64)], next_forward_token: Option<String>) -> GeloLogEventsResponse {
    GeloLogEventsResponse {
      events: events
        .iter()
        .map(|e| LogEvent {
          message: e.0.to_string(),
          timestamp: e.1,
        })
        .collect(),
      next_forward_token: next_forward_token,
    }
  }
}
