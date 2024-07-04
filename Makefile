all:
	cargo run > image.ppm

build:
	cargo build

clean:
	cargo clean
	rm -rf image.ppm

image_clean:
	rm -rf *.ppm