# scraper makefile

TAG = cezarmathe/usscraper
VERSION = 0.1.2

package:
	docker build \
		--tag "$(TAG):$(VERSION)" \
		.
.PHONY: package

push: package
	docker push "$(TAG):$(VERSION)"
.PHONY: push

save-image: package
	docker image save \
		-o "../shared/artifacts/$(shell printf "%s" "$(TAG)" | cut -d '/' -f 2)-$(VERSION).tar" \
		"$(TAG):$(VERSION)"
.PHONY: save-image
