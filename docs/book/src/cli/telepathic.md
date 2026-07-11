# telepathic

The verbosity settings for the cli

```bash
$ telepathic --help
Usage: telepathic [OPTIONS] <COMMAND>

Commands:
  run   Run Telepathic
  db    Telepathic database commands
  help  Print this message or the help of the given subcommand(s)

Options:
      --telepathic-db-path <TELEPATHIC_DB_PATH>
          path to the telepathic libmdbx db

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Display:
  -v, --verbosity...
          Set the minimum log level.

          -v      Errors
          -vv     Warnings
          -vvv    Info
          -vvvv   Debug
          -vvvvv  Traces (warning: very verbose!)

      --quiet
          Silence all log output

      --metrics-port <METRICS_PORT>
          [default: 6923]

      --skip-prometheus
```
