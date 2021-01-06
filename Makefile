# unisuite makefile

COMPONENTS = scraper watchman

all: dev

dev:
	./scripts/artifacts.sh dev
	cd terraform; make; cd ..
.PHONY: dev

prod:
	./scripts/artifacts.sh prod
	cd terraform; make deploy ENVIRONMENT=prod; cd ..
.PHONY: prod
