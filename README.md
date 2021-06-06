# weatherlogger

You need an apikey from https://openweathermap.org/
uses a file of locations and can optionally instead use an ip location

To run without installing:

```sh
cargo run
```

To run the tests:

```sh
cargo test -- --test-threads 1
```

To install clone repo and run in folder:

```sh
cargo install --path .
```

To run with config file:

```sh
weatherlogger -c </path/to/config.json>
```
Example config file can be found in 'config_example.json'

Optionally to use ip location run:

```sh
weatherlogger -c </path/to/config.json> -i
```

WIP: Creating graphs from stored data
```sh
weatherlogger -p
```
Will plot the temperature of a location which locations will be configured later
