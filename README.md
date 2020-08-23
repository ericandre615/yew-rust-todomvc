# TodoMVC
## Rust + Yew

A simple demo todo app using the Rust language with the Yew framework

There is a package.json in the root that is ~~current only for running commands~~ for some basic commands and webpack-dev-server
- `npm run build` to build using `wasm-pack`
- `npm start` will start `webpack-dev-server`
 - (could potentially also work with out the dependencies/config with `--single` option of `serve`)
 - This would
you can also run `wasm-pack` with `wasm-pack build --target web --out-name wasm --out-dir ./static/build`
and use `http-simple-server` or whatever else you want to serve up `static` on `localhost` but one that has the ability to either config for single page app
or can allow config to rewrite all routes to serve `index.html` with a status of `200` (not `404`)

You will need `nodejs`/`npm`, `rust`, and `wasm-pack` all installed
