build:
	@cargo build

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline -- --color=always --test-threads=1 --nocapture

mongostart:
	@docker run -d -p 27017:27017 -v `pwd`/data/db:/data/db --name book_rust_mongodb mongo

mongostop:
	@docker stop book_rust_mongodb

docs: build
	@cargo doc --no-deps

dev:
	@cargo run