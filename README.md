# TodoMVC
## Rust + Yew

A simple demo todo app using the Rust language with the Yew framework

There is a package.json in the root that is current only for running commands
- `npm run build` to build using `wasm-pack`
- `npm start` will `serve` the `static` folder
 
you can also run `wasm-pack` with `wasm-pack build --target web --out-name wasm --out-dir ./static/build`
and use `http-simple-server` or whatever else you want to serve up `static` on `localhost`

## TODOs
- Implement `yew-router` for filter states (all, complete, active)
- Wipe input on pressing `Enter`/add-todo
- Figure out the issue with the way listener for pressing enter and the change of input work. I always have to press Enter twice to add-todo

