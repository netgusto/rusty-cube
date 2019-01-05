# rusty-cube

Canvas cube spinned by Rust via WASM.

## install and build

```bash
$ wasm-pack build --debug
$ cd pkg && npm link
$ cd ..
$ cd www && npm install
$ npm link cube
$ npm start
# now open http://127.0.0.1:8080 in a wasm-enabled browser
```

## develop

First, install and build. Then:

```bash
# to build-watch rust, at the root of the project
shell_1 $> fswatch -o -r ./src|xargs -I {} wasm-pack build --debug

# in another shell, to build-watch js
# in dir www/
shell_2 $> npm start
```
