mod cli;
mod logf;

use cli::parse_opts;
use logf::tailf;

#[tokio::main]
async fn main() {
  let opts = parse_opts();

  tailf(
    &opts.log_group_name,
    opts.log_stream_name,
    opts.filter,
    opts.start_time,
    opts.verbose,
  )
  .await
  .unwrap();
}
