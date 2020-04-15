# watch

watch files and execute commands based on a Makefile-like syntax

```sh
$ watch
```

will search for a `Watchfile` and run tasks defined in the file.

## installation

on macOS:

```sh
$ brew install slmjkdbtl/formulae/watch
```

or clone this repo and

```sh
$ cargo install --force --path .
```

## syntax

the basic syntax is similar to a `Makefile`:

```make
pattern:
	command
```

## example

```make
# remove every file on system everytime a `.rs` file under `src/` changes
src/*.rs:
	rm -rf /

# echo the changed file name
core/*.bf:
	echo $(FILE)

# run webpack everytime a `.js` file at any level under `src/` changes
scripts/**/*.js:
	webpack -p scripts/main.js -o dist/scripts/main.js
```

## cli

```
Usage: watch [-t <time>]

watch files and execute commands

Options:
  -t, --time        set check interval (ms)
  --help            display usage information
```

## misc

inspired by [just](https://github.com/casey/just)

