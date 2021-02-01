# usdiff makefile

TAG = cezarmathe/usdiff
VERSION = 0.1.0

package:
	cp "../target/x86_64-unknown-linux-musl/$(shell if [[ "$(VERSION)" != "dev" ]]; then printf "%s" "release"; else printf "%s" "debug"; fi)/usdiff" .
	docker build \
		--tag "$(TAG):$(VERSION)" \
		.
	rm ./usdiff
.PHONY: package

push: package
	docker push "$(TAG):$(VERSION)"
.PHONY: push

save-image: package
	docker image save \
		-o "../shared/artifacts/$(shell printf "%s" "$(TAG)" | cut -d '/' -f 2)-$(VERSION).tar" \
		"$(TAG):$(VERSION)"
.PHONY: save-image
