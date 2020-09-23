NAME := nlsd
PROFILE ?= debug
TARGET ?=
ALPINE_DOCKER_IMAGE ?= nlsd-py-alpine-builder

ifeq ($(PROFILE),release)
RELEASE_FLAG := --release
else
RELEASE_FLAG :=
endif

ifeq ($(TARGET),)
TARGET_FLAG :=
TARGET_DIR := target
else
TARGET_FLAG := --target $(TARGET)
TARGET_DIR := target/$(TARGET)
endif

build: venv
	cargo build $(RELEASE_FLAG) $(TARGET_FLAG)
	cp $(TARGET_DIR)/$(PROFILE)/lib$(NAME).so $(TARGET_DIR)/$(PROFILE)/$(NAME).so
	for d in $$(ls venv/lib | grep python); do \
		cp $(TARGET_DIR)/$(PROFILE)/$(NAME).so venv/lib/$$d/site-packages/$(NAME).so; \
	done
.PHONY: build

docker-build:
	docker build -f docker/Dockerfile.alpine --build-arg LIBNAME=$(NAME) \
		-t $(ALPINE_DOCKER_IMAGE) .
	mkdir -p target/x86_64-alpine-linux-musl/release
	docker run --rm  -v $(shell pwd)/target/x86_64-alpine-linux-musl/release:/tmp/target \
		$(ALPINE_DOCKER_IMAGE) cp /build/$(NAME).so /tmp/target/$(NAME).so
.PHONY: docker-build

github-release: docker-build
	cargo build --target x86_64-unknown-linux-gnu
	cargo build --target x86_64-unknown-linux-gnu --release
	mkdir -p target/github-release
	cp target/x86_64-alpine-linux-musl/release/$(NAME).so \
		target/github-release/$(NAME)-x86_64-alpine-linux-musl.so
	cp target/x86_64-unknown-linux-gnu/release/lib$(NAME).so \
		target/github-release/$(NAME)-x86_64-unknown-linux-gnu.so
	cp target/x86_64-unknown-linux-gnu/debug/lib$(NAME).so \
		target/github-release/$(NAME)-x86_64-unknown-linux-gnu.debug.so
.PHONY: release

check:
	cargo check $(RELEASE_FLAG) $(TARGET_FLAG)
.PHONY: check

venv:
	python3 -m virtualenv venv
	. venv/bin/activate && python -m pip install -r dev-requirements.txt

test: build
	. venv/bin/activate && python -m pytest
.PHONY: test
