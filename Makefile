.PHONY : build server dist

package = ai_wargame_web

build:
	wasm-pack build --target web

server:
	python -m http.server

server3:
	python3 -m http.server

dist:
	mkdir -p dist/
	rm dist/*.zip
	cp index.html dist/
	mkdir -p dist/pkg
	cp pkg/$(package)_bg.wasm pkg/$(package).js dist/pkg/
	cd dist; zip -9r $(package).zip index.html pkg
	rm -rf dist/index.html dist/pkg
