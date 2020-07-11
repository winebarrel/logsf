mod cli;
mod logf;

use cli::parse_opts;
use logf::tailf;
use rusoto_core::Region;
use rusoto_logs::CloudWatchLogsClient;
use std::io;

#[tokio::main]
async fn main() {
  let opts = parse_opts();
  let client = CloudWatchLogsClient::new(Region::default());
  let out = io::stdout();

  tailf(
    &client,
    &opts.log_group_name,
    opts.log_stream_name,
    opts.filter,
    opts.start_time,
    opts.verbose,
    &mut out.lock(),
    Some(opts.wait),
  )
  .await
  .unwrap();
}
