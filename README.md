# FWATCH

A small CLI that polls a directory for a file matching a glob pattern, then exits as soon as one appears (or when a deadline passes).

## Usage

```
fwatch <path> <pattern> <rundays> <endtime> [-z <timezone>] [-i <interval>]
```

### Positional arguments

| Arg       | Description                                                                  |
|-----------|------------------------------------------------------------------------------|
| `path`    | Directory to watch (must exist).                                             |
| `pattern` | Filename or glob to look for, e.g. `*.done`, `report_*.csv`.                 |
| `rundays` | Maximum number of days to keep watching (0–365).                             |
| `endtime` | Wall-clock cutoff on the final day, `HH:MM:SS` in 24-hour format.            |

### Options

| Flag                | Default     | Description                                                  |
|---------------------|-------------|--------------------------------------------------------------|
| `-z`, `--timezone`  | `Etc/UTC`   | IANA timezone for `endtime`, e.g. `America/New_York`.        |
| `-i`, `--interval`  | `5`         | Poll interval in seconds.                                    |

## Behavior

- On every tick, `fwatch` globs `<path>/<pattern>` and returns the first regular file it finds.
- On match: prints the matched path to **stdout** and exits `0`.
- On deadline (current time ≥ `rundays` days from now at `endtime` in `timezone`): exits `2`.
- On bad args or scan error: exits `1`.

Status messages are written to **stderr** so the matched path on stdout stays clean for pipelines.

## Example

Watch `/data/incoming` for `report_*.csv` every 10 seconds, giving up at 17:00 New York time tomorrow:

```sh
fwatch /data/incoming 'report_*.csv' 1 17:00:00 -z America/New_York -i 10
```

Use the exit code in a script:

```sh
if path=$(fwatch /data/incoming '*.done' 0 23:59:59); then
    echo "got $path"
else
    echo "timed out or errored ($?)"
fi
```
