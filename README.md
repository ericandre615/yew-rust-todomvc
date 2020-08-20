# TodoMVC
## Rust + Yew

A simple demo todo app using the Rust language with the Yew framework

There is a package.json in the root that is current only for running commands
- `npm run build` to build using `wasm-pack`
- `npm start` will `serve` the `static` folder
 
you can also run `wasm-pack` with `wasm-pack build --target web --out-name wasm --out-dir ./static`
and use `http-simple-server` or whatever else you want to serve up `static` on `localhost`

## TODOs
- Figure out while `Children` as a prop don't update on change/render
- Implement `yew-router` for filter states (all, complete, active)
