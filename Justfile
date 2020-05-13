# wengwengweng

name := "watch"

run +args="":
	cargo run -- {{args}}

build:
	cargo build --release

install:
	cargo install --force --path .

pack:
	rm -rf dist
	mkdir -p dist
	cp target/release/{{name}} dist/{{name}}
	upx dist/{{name}}
	zip dist/{{name}}-x86_64-apple-darwin.zip dist/{{name}}
	rm dist/{{name}}
	sha256sum dist/{{name}}-x86_64-apple-darwin.zip

doc crate="watch":
	cargo doc --no-deps --open -p {{crate}}

update:
	cargo update

bloat:
	cargo bloat --release --crates

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

depgraph:
	cargo deps --all-deps | dot -Tpng > $TMPDIR/graph.png; \
		open $TMPDIR/graph.png

