# Based on CargoMake from https://github.com/neosmart/CargoMake

COLOR ?= always # Valid COLOR options: {always, auto, never}
RUSTC ?= rustc
# We don't use the command variables because they may contain args.
# The could be overridden
REQ_TOOLS ?= rustc cargo git python3 zip

ifeq ($(OS),Windows_NT)
	# For Cygwin/MsysGit Windows Compatibility.
	CARGO?=HOME=$(USERPROFILE) cargo --color $(COLOR)
	BIN_SUFFIX?=.exe
else
	# For Linux/OSX
	CARGO?=cargo --color $(COLOR)
	BIN_SUFFIX?=
endif

ifndef SKIP_TOOL_CHECK
	# Check for required tools.
	# https://stackoverflow.com/a/25668869/260740
	K := $(foreach exec,$(REQ_TOOLS),\
			$(if $(shell which $(exec) 2> /dev/null),some string,$(error "Required tool '$(exec)' was not found in PATH. Install it or set `SKIP_TOOL_CHECK=1` to force.")))
endif

# TODO: Allow this to be blank if python is not installed
TARGET=$(shell RUSTC_BOOTSTRAP=1 $(RUSTC) -Z unstable-options --print target-spec-json | python3 -c 'import json,sys;obj=json.load(sys.stdin);print(obj["llvm-target"])')
GIT_BIN?=git


PROJECT =  $(shell grep '^name' odel/Cargo.toml | grep -Po '(?<=")[^"]+(?=")')
VERSION = $(shell grep '^version' odel/Cargo.toml | grep -Po '(?<=")[^"]+(?=")')

GITHUB_TAG=v$(VERSION)

DISTROOT=./dist
DISTDIR = $(PROJECT)-$(VERSION)-$(TARGET)
DISTZIP=$(PROJECT)-$(VERSION)-$(TARGET)-bin.zip
SRCDISTZIP=$(PROJECT)-$(VERSION)-src.zip

.PHONY: all bench build release check clean doc install publish run test update

all: build

bench: ## Runs benchmark tests
	@$(CARGO) bench

build: ## Builds the project
	@$(CARGO) build

release: ## Builds the optimized released version of the project
	@$(CARGO) build --release

check: build test ## Checks the project and all of its dependencies for errors.

clean: ## Removes all generated files
	@$(CARGO) clean

doc: ## Generates documentation HTML for this project and all dependencies
	@$(CARGO) doc

install: build ## Installs the project binary to the cargo bin folder
	@$(CARGO) install

publish: ## Uploads the crate to rust package registry
	@$(CARGO) publish

run: build ## Runs the project executable
	@$(CARGO) run

test: build ## Runs any unit tests
	@$(CARGO) test

clippy: build ## Runs clippy code inspector
	@$(CARGO) clippy

update: ## Updates dependencies in the Cargo.lock file to the latest version.
	@$(CARGO) update

expand: ## Expands all macros in the source files and prints to STDOUT
	@$(CARGO) expand --color=always --theme=1337

completion: release ## Generates autocompletion scripts
	mkdir -p target/release/completion
	./target/release/$(PROJECT)$(BIN_SUFFIX) --shell-completion zsh > target/release/completion/zsh
	./target/release/$(PROJECT)$(BIN_SUFFIX) --shell-completion bash > target/release/completion/bash
	./target/release/$(PROJECT)$(BIN_SUFFIX) --shell-completion fish > target/release/completion/fish
	./target/release/$(PROJECT)$(BIN_SUFFIX) --shell-completion powershell >target/release/completion/powershell
	./target/release/$(PROJECT)$(BIN_SUFFIX) --shell-completion elvish > target/release/completion/elvish

distclean: clean ## Cleans all generated files, including distribution files
	rm -rf $(DISTROOT)

dist: release completion ## Create a distribution package
	rm -rf $(DISTROOT)/$(DISTDIR)
	mkdir -p $(DISTROOT)/$(DISTDIR)
	cp target/release/$(PROJECT)$(BIN_SUFFIX) $(DISTROOT)/$(DISTDIR)
	-cp README.rst $(DISTROOT)/$(DISTDIR)
	-cp ChangeLog.rst $(DISTROOT)/$(DISTDIR)
	cp -r target/release/completion $(DISTROOT)/$(DISTDIR)
	cd $(DISTROOT) && zip -r $(DISTZIP) $(DISTDIR)/
	rm -rf $(DISTROOT)/$(DISTDIR)

ChangeLog.$(VERSION).rst: ChangeLog.rst
	test $(shell grep -c '^v$(VERSION)' ChangeLog.rst) -eq 1 || (echo "Please add a change log entry for release $(VERSION) before distributing"; exit 1)
	sed '1,/^v.*/d;/^-\+$$/d;/^v.*/,$$d' ChangeLog.rst > $@

github-publish: ChangeLog.$(VERSION).rst dist
	# Check if the tag exists in the local repo
	test $(shell git tag -l | grep -x -c -F "$(GITHUB_TAG)") -eq 1 || ( printf "The tag $(GITHUB_TAG) does not exit in this repository. Tag your release first.\n\tgit tag $(GITHUB_TAG)\n"; exit 1 )
	# Check if the tag exists in the remote repo
	test $(shell git ls-remote --tags origin | grep -c "refs/tags/$(GITHUB_TAG)$$") -eq 1 || ( echo "The tag $(GITHUB_TAG) has not been pushed to remote. Push your tags first."; exit 1 )
	# Create release
	gh release create $(GITHUB_TAG) -F ChangeLog.$(VERSION).rst $(DISTROOT)/$(DISTZIP)


distsrc: ## Creates a zip file with source code
	mkdir -p $(DISTROOT)/$(DISTDIR)
	rm -f $(DISTROOT)/$(SRCDISTZIP)
	$(GIT_BIN) archive --format zip --output $(DISTROOT)/$(SRCDISTZIP) HEAD

help: ## This help dialog.
	@IFS=$$'\n' ; \
	help_lines=(`fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##/~~/'`); \
	for help_line in $${help_lines[@]}; do \
		IFS=$$'~~' ; \
		help_split=($$help_line) ; \
		IFS=$$':' ; \
		help_command_split=($${help_split[0]}) ; \
		help_command=`echo $${help_command_split[0]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		help_target=`echo $${help_command_split[1]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		help_info=`echo $${help_split[2]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		printf '\033[36m'; \
		printf "%-9s %s" $$help_command ; \
		printf '\033[33m'; \
		printf ": %-35s %s" $$help_target ; \
		printf '\033[0m'; \
		printf "%s\n" $$help_info; \
	done

# Suppress command echo by default. When V=<anything> disable suppression.
$(V).SILENT:
	
