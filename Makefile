test:
	docker run --rm -v ${PWD}:/build -w /build -t liuchong/rustup:nightly \
		sh -c "cargo test"
