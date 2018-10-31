.DEFAULT_GOAL := help

# -- context --
# location for js binaries
jbin = ./node_modules/.bin

# -- run/stop --
## starts the dev server
start:
	$(jbin)/ts-node-dev --no-notify src/index.ts
## kills the running dev server
stop:
	lsof -t -i:3000 | xargs kill

.PHONY: start stop

# -- help --
help:
	@awk "$$HELP" $(MAKEFILE_LIST)

define HELP
BEGIN {
	print "\033[;1musage:\033[0m";
	print "  make <command>\n";
	print "\033[;1mcommands:\033[0m";
}
/^## (.*)$$/ {
	$$1=""; docs=$$0;
	getline;
	sub(/:/, "", $$1);
	printf "  \033[36m%-5s\033[0m %s\n", $$1, docs;
}
endef
export HELP

.PHONY = help
