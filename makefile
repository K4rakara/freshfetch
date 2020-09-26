all: cargo pkg

cargo:
	@echo "Compiling...";
	@cargo build --release -vv;
	@echo "Compiled.";

pkg:
	@echo "Packaging...";
	@mkdir -p ./pkg/;
	@install -Dm755 \
		./target/release/freshfetch \
		./pkg/usr/bin/freshfetch;
	@tar -zcvf freshfetch.tar.gz -C pkg .;
	@echo "Packaged.";

