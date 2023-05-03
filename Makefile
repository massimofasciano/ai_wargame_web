.PHONY : build server dist

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
	cp pkg/ai_wargame_web_console_bg.wasm pkg/ai_wargame_web_console.js dist/pkg/
	cd dist; zip -9r ai_wargame_web_console.zip index.html pkg
	rm -rf dist/index.html dist/pkg
