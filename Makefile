# Makefile

# Variables
DOCKER_IMAGE_NAME := capital-gains
OUTPUT_DIR := ./output
DOCKERFILE := Dockerfile
BINARY_NAME := capital-gains

.PHONY: all build clean test

all: build

clean:
	@echo "Cleaning up..."
	rm -rf $(OUTPUT_DIR)
	docker rmi $(DOCKER_IMAGE_NAME) 2>/dev/null || true

test:
	@echo "Running tests..."
	docker build --target builder -t $(DOCKER_IMAGE_NAME)-test -f $(DOCKERFILE) .
	docker run --rm $(DOCKER_IMAGE_NAME)-test cargo test
	docker rmi $(DOCKER_IMAGE_NAME)-test 2>/dev/null || true

help:
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build    - Build the Rust application (default)"
	@echo "  clean    - Remove built binary and Docker image"
	@echo "  test     - Run the tests"
	@echo "  help     - Show this help message"
