# dodona-containerfile-evaluator

dodona-containerfile-evaluator is part of the [Dodona Docker judge](https://github.com/dodona-edu/judge-docker).
It's responsible for parsing the Containerfile and checking if it fulfills the requirements set by the config file.

```
Usage: dodona-containerfile-evaluator --config <path> <Containerfile>

Arguments:
  <Containerfile>  Containerfile to operate on

Options:
  -c, --config <path>  Sets a custom config file
  -h, --help           Print help
  -V, --version        Print version
```

Example config file:
```json
{
  "from": {
    "image": "alpine",
    "tag": "3:20"
  },
  "user": "runner",
  "workdir": "/course",
  "comments": [ "docker run" ]
}
```
