protodoc:
	protoc --doc_out=./doc --doc_opt=html,index.html proto/**/*.proto

build:
	cargo build

format:
	cargo fmt

fix:
	cargo fix

lint:
	cargo clippy
