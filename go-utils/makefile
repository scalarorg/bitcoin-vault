
.PHONY: proto-gen

DOCKER := docker

PROTO_GEN_IMAGE := go-utils/docker/protogen

proto-gen: proto-clean
	@echo "Generating Protobuf files"
	@if ! docker images $(PROTO_GEN_IMAGE) | grep -q $(PROTO_GEN_IMAGE); then \
		DOCKER_BUILDKIT=1 docker build -t $(PROTO_GEN_IMAGE) -f ./docker/dockerfile.protogen .; \
	fi

	@$(DOCKER) run --rm -v $(CURDIR):/workspace --workdir /workspace $(PROTO_GEN_IMAGE) sh ./scripts/protogen.sh

proto-clean:
	@rm -rf ./**/*.pb.go

