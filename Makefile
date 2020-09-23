NAME := nlsd
PROFILE ?= debug
# TARGET ?= x86_64-unknown-linux-gnu

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

check:
	cargo check $(RELEASE_FLAG) $(TARGET_FLAG)
.PHONY: check

venv:
	python3 -m virtualenv venv
	. venv/bin/activate && python -m pip install -r dev-requirements.txt

test: build
	. venv/bin/activate && python -m pytest
