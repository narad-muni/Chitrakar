.PHONY: rs cpp build-rs build-cpp

rsw:
	cd $(CURDIR)/rs && cargo watch -c -w src -x run
rs:
	cd $(CURDIR)/rs && cargo run
build-rs:
	cd $(CURDIR)/rs && cargo build --release
cpp:
	cd $(CURDIR)/cpp && g++ main.cpp -o main && ./main
build-cpp:
	cd $(CURDIR)/cpp && g++ main.cpp -o main 