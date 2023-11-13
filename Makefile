check:
	cargo clippy

doc:
	cargo doc --workspace --open

install:
	cargo install --path .

readme:
	cargo readme > README.md

publish: readme
	cargo publish
