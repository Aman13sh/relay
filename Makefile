# Makefile for relay
#
# Builds release binaries and installs them to PREFIX (default: /usr/local).
# Targets:
#   make build     — compile release binaries
#   make install   — build + copy to PREFIX/bin
#   make uninstall — remove installed binaries
#   make clean     — delete build artifacts
#   make package   — create a distributable .tar.gz

PREFIX     ?= /usr/local
BINDIR      = $(PREFIX)/bin
BUILD_DIR   = target/release
SERVER_BIN  = relay-server
CLIENT_BIN  = relay-client
VERSION    ?= $(shell grep '^version' crates/server/Cargo.toml | head -1 | cut -d'"' -f2)
ARCHIVE     = relay-$(VERSION)-$(shell uname -s | tr '[:upper:]' '[:lower:]')-$(shell uname -m).tar.gz

.PHONY: build install uninstall clean package

# Compile both binaries in release mode (optimised, no debug info).
build:
	cargo build --release

# Build then copy binaries to PREFIX/bin.
# Creates the directory if it does not exist.
install: build
	install -d $(BINDIR)
	install -m 755 $(BUILD_DIR)/$(SERVER_BIN) $(BINDIR)/$(SERVER_BIN)
	install -m 755 $(BUILD_DIR)/$(CLIENT_BIN) $(BINDIR)/$(CLIENT_BIN)
	@echo ""
	@echo "Installed:"
	@echo "  $(BINDIR)/$(SERVER_BIN)"
	@echo "  $(BINDIR)/$(CLIENT_BIN)"

# Remove the installed binaries from PREFIX/bin.
uninstall:
	rm -f $(BINDIR)/$(SERVER_BIN)
	rm -f $(BINDIR)/$(CLIENT_BIN)
	@echo "Uninstalled relay-server and relay-client from $(BINDIR)"

# Delete all build artifacts produced by cargo.
clean:
	cargo clean

# Build then bundle binaries + README into a versioned tar.gz for distribution.
# Output: relay-<version>-<os>-<arch>.tar.gz
package: build
	mkdir -p dist/relay-$(VERSION)/bin
	cp $(BUILD_DIR)/$(SERVER_BIN) dist/relay-$(VERSION)/bin/
	cp $(BUILD_DIR)/$(CLIENT_BIN) dist/relay-$(VERSION)/bin/
	cp mdfiles/README.md           dist/relay-$(VERSION)/README.md
	cp install.sh                  dist/relay-$(VERSION)/install.sh
	chmod +x                       dist/relay-$(VERSION)/install.sh
	tar -czf $(ARCHIVE) -C dist relay-$(VERSION)
	rm -rf dist
	@echo ""
	@echo "Package ready: $(ARCHIVE)"
