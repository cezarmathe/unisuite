# unisuite makefile

COMPONENTS = scraper watchman

all: dev

dev:
	cross build --target x86_64-unknown-linux-musl
	./scripts/artifacts.sh dev
	cd terraform; make; cd ..
.PHONY: dev

prod:
	cross build --target x86_64-unknown-linux-musl --release
	./scripts/artifacts.sh prod
	cd terraform; make deploy ENVIRONMENT=prod; cd ..
.PHONY: prod
