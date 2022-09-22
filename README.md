# kvdb

A simple key-value database.

## Usage

#### Start a kvdb interactive shell

```
$ kvdb ./some-file
```

#### Set a key-value pair

```
kvdb> set foo bar
```

#### Get a value by key

```
kvdb> get foo
bar
```

#### Delete a key-value pair

```
kvdb> del foo
```

## Docker usage

#### Build the image

```
$ docker build -t kvdb .
```

#### Start a kvdb interactive shell

```
$ docker run -it --rm -v $(pwd)/data:/data kvdb
```

The docker image uses `/data/data.db` as the database file.
