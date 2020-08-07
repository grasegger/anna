.PHONY: example serve-example

example:
	cd example &&  wasm-pack build --target no-modules --out-dir dist

serve-example: example
	docker run --rm -it -p 8080:80 -v ${shell pwd}/example/dist:/usr/local/apache2/htdocs/ httpd