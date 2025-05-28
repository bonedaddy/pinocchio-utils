.PHONY: lint
lint:
	cargo fmt --all
	cargo clippy --workspace --all-targets

.PHONY: build-docker-debug
build-docker-debug:
	DOCKER_BUILDKIT=1 docker \
		build \
		--build-arg BUILDKIT_INLINE_CACHE=1 \
		-t muchproject/very-image-debug:latest \
		-f Dockerfile.dev \
		.

.PHONY: build-docker-release
build-docker-release:
	DOCKER_BUILDKIT=1 docker \
		build \
		--build-arg BUILDKIT_INLINE_CACHE=1 \
		-t muchproject/very-image:latest \
		-f Dockerfile \
		.
