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

To run with apikey and file of locations:

```sh
weatherlogger -a </path/to/apikey> -l </path/to/locations_file/>
```

Optionally to use ip location run:

```sh
weatherlogger -a </path/to/apikey> -i
```
