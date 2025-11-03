default:
	@just --list

[no-cd]
run *FLAGS:
	cargo run -- {{ FLAGS }}

test:
	cargo test

publish: test
	cargo publish
