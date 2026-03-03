default:
	@just --list

[no-cd]
run *FLAGS:
	cargo run --quiet -- {{ FLAGS }}

test:
	cargo test

publish: test
	cargo publish
