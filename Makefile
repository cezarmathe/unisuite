# unisuite makefile

COMPONENTS = scraper watchman

# Cargo command to use for building.
# You can use either cargo or cross.
CARGO = cargo

all: dev

dev:
	$(CARGO) build --target x86_64-unknown-linux-musl
	./scripts/artifacts.sh dev
	cd terraform; make; cd ..
	docker image prune --force
.PHONY: dev

prod:
	$(CARGO) build --target x86_64-unknown-linux-musl --release
	./scripts/artifacts.sh prod
	cd terraform; make deploy ENVIRONMENT=prod; cd ..
	docker image prune --force
.PHONY: prod

sshfs-mount:
	mkdir -p ./shared/data
	sshfs -p 2222 vagrant@127.0.0.1:/srv/file ./shared/data
.PHONY: sshfs-mount

sshfs-umount:
	fusermount3 -u ./shared/data
.PHONY: sshfs-umount
