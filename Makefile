default:
	@# Check if rust is installed
	@if [ -d $$(echo ~/.cargo) ]; then \
		make -s compile; \
	else \
		make -s install-rust; \
	fi

compile:
	@cargo build --release

install-rust:
	@echo "Installing rust in ~/.cargo"

	@# This command can look super sketchy, but it is a standard way to install
	@# Rust without a package manager: https://www.rust-lang.org/tools/install
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

run:
	@cargo run --release
