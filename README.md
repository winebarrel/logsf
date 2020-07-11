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
