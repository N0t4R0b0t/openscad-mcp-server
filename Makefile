.PHONY: build release test fmt clippy check clean help install

help:
	@echo "OpenSCAD MCP Server - Development Commands"
	@echo ""
	@echo "  make build       Build debug binary"
	@echo "  make release     Build optimized release binary"
	@echo "  make test        Run tests"
	@echo "  make fmt         Format code with rustfmt"
	@echo "  make clippy      Run clippy linter"
	@echo "  make check       Run cargo check"
	@echo "  make clean       Remove build artifacts"
	@echo "  make install     Install to ~/.local/bin/"
	@echo "  make help        Show this help message"

build:
	cargo build

release:
	cargo build --release

test:
	cargo test --verbose

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

check:
	cargo check

clean:
	cargo clean

install: release
	mkdir -p ~/.local/bin
	cp target/release/openscad-mcp-server ~/.local/bin/
	chmod +x ~/.local/bin/openscad-mcp-server
	@echo "Installed to ~/.local/bin/openscad-mcp-server"
	@echo "Make sure ~/.local/bin is in your PATH"
