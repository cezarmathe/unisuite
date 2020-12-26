# scraper makefile

DOCKER_TAG = cezarmathe/usscraper
VERSION = 0.1.0

build:
	docker build \
		--tag "$(DOCKER_TAG):$(VERSION)" \
		.
.PHONY: build

push:
    docker push "$(DOCKER_TAG):$(VERSION)"
.PHONY: push

run:
	docker run \
		--env-file .env \
		--name usscraper \
		--rm \
		--tty \
		--volume "$(shell pwd)/data:/var/usscraper" \
		"$(DOCKER_TAG):$(VERSION)"

run-shell:
	docker run \
		--entrypoint /bin/bash \
		--env-file .env \
		--interactive \
		--name usscraper \
		--rm \
		--tty \
		--volume "$(shell pwd)/data:/var/usscraper" \
		"$(DOCKER_TAG):$(VERSION)"
