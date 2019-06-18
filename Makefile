serve: dev
	python3 -m http.server --directory dist

dev: pages webpack

.PHONY: pages
pages:
	cargo run

.PHONY: webpack
webpack:
	npm run dev
