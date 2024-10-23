.PHONY: compose test clean bench

compose:
	docker compose up -d
test:
ifeq ($(and $(m),$(f)),)
	RUST_LOG=debug cargo test --lib -- --nocapture
else
	RUST_BACKTRACE=1 RUST_LOG=debug cargo test --lib $(m)::tests::$(f) -- --nocapture
endif
