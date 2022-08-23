target=/usr/local/bin

.PHONY: all
all: build install

.PHONY: build
build:
	@echo "Building release..."
	@cargo build --release

.PHONY: setup
install:
	@echo "Installing svg2svelte to $(target)"
	@sudo cp ./target/release/svg2svelte $(target)

.PHONY: clean
clean:
	@echo "Removing svg2svelte from $(target)"
	@sudo rm $(target)/svg2svelte
