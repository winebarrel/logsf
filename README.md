# logsf

logsf is a tool to follow tail messages of CloudWatch Logs.

[![Build Status](https://travis-ci.org/winebarrel/logsf.svg?branch=master)](https://travis-ci.org/winebarrel/logsf)

## Usage

```
Usage: logsf [options]

Options:
    -g, --log-group-name NAME
                        log group name
    -s, --log-stream-name NAME
                        log stream name
    -f, --stream-filter REGEX
                        log stream filter regex
    -t, --start-time TIME
                        event start time
    -w, --wait SEC      loop interval sec
    -V, --verbose       verbose output
    -v, --version       print version and exit
    -h, --help          print usage and exit
```

```
$ logsf -g my-group -s my-stream -V
2020-05-13T01:51:18+09:00 foo
2020-05-13T01:51:18+09:00 bar
2020-05-13T01:51:19+09:00 zoo
...
$ logsf -g my-group -f ^stream-prefix- -V
stream-prefix-1 2020-05-13T01:51:18+09:00 foo
stream-prefix-2 2020-05-13T01:51:18+09:00 bar
stream-prefix-1 2020-05-13T01:51:19+09:00 zoo
...
```
