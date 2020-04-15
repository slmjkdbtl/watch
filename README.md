# watch

`watch` watches files and executes commands based on a Makefile-like syntax

running

```sh
$ watch
```

will search for a `Watchfile` in current directly and run tasks defined in the file.

## syntax

the basic syntax is similar to a `Makefile`:

```make
pattern:
	command
```

e.g.

```make
src/*.rs:
	rm -rf /
```

remove every file on system everytime a `.rs` file under `src/` changes

```make
src/**/*.rs:
	echo $(FILE)
```
echoes the path of the changed file everytime a `.rs` file at any level under `src/` changes

## cli

## misc
inspired by [just](https://github.com/casey/just)

