extern crate dtparse;
extern crate getopts;
extern crate regex;

use regex::Regex;
use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Options {
  pub log_group_name: String,
  pub log_stream_name: Option<String>,
  pub stream_filter: Option<Regex>,
  pub start_time: Option<i64>,
  pub verbose: bool,
  pub wait: u64,
}

fn print_usage(program: &str, opts: getopts::Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

pub fn parse_opts() -> Options {
  let args: Vec<String> = env::args().collect();
  let program = &args[0];
  let mut opts = getopts::Options::new();

  opts.optopt("g", "log-group-name", "log group name", "NAME");
  opts.optopt("s", "log-stream-name", "log stream name", "NAME");
  opts.optopt("f", "stream-filter", "log stream filter regex", "REGEX");
  opts.optopt("t", "start-time", "event start time", "TIME");
  opts.optopt("w", "wait", "loop interval sec", "SEC");
  opts.optflag("V", "verbose", "verbose output");
  opts.optflag("v", "version", "print version and exit");
  opts.optflag("h", "help", "print usage and exit");

  let matches = opts.parse(&args[1..]).unwrap();

  if matches.opt_present("h") {
    print_usage(&program, opts);
    process::exit(0)
  }

  if matches.opt_present("v") {
    println!("{}", VERSION);
    process::exit(0)
  }

  let stream_filter = match matches.opt_str("f") {
    Some(re) => Some(Regex::new(&re).unwrap()),
    None => None,
  };

  let start_time = match matches.opt_str("t") {
    Some(time_str) => Some(dtparse::parse(&time_str).unwrap().0.timestamp_millis()),
    None => None,
  };

  Options {
    log_group_name: matches.opt_str("g").unwrap(),
    log_stream_name: matches.opt_str("s"),
    stream_filter: stream_filter,
    start_time: start_time,
    verbose: matches.opt_present("V"),
    wait: matches.opt_get_default("w", 1).unwrap(),
  }
}
