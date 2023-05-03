.PHONY : build server dist

build:
	wasm-pack build --target web

server:
	python -m http.server

dist:
	mkdir -p dist/
	cp index.html dist/
	mkdir -p dist/pkg
	cp pkg/*.wasm pkg/*.js dist/pkg/
	cd dist; zip -9r ai_wargame_web.zip index.html pkg
	rm -rf dist/index.html dist/pkg
	