.PHONY : build server dist

package = ai_wargame_web

build:
	wasm-pack build --target web

server:
	python -m http.server

server3:
	python3 -m http.server

broker:
	python game_broker.py

broker3:
	python3 game_broker.py

dist: build
	mkdir -p dist/
	rm dist/*.zip
	cp index.html game.js game.css dist/
	mkdir -p dist/pkg
	cp pkg/$(package)_bg.wasm pkg/$(package).js dist/pkg/
	cd dist; zip -9r $(package).zip index.html game.js game.css pkg
	rm -rf dist/index.html dist/game.js dist/game.css dist/pkg
