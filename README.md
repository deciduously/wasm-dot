# wasm-dot

Demo application of a simple wasm app in Rust.

## Usage

```
$ git clone git@github.com:deciduously/wasm-dot.git
$ cd wasm-dot
../wasm-dot $ wasm-pack build
[INFO]: Checking for the Wasm target...
...blah blah blah
[INFO]: :-) Your wasm pkg is ready to publish at /home/ben/code/wasm-dot/pkg.
../wasm-dot $ cd www
../wasm-dot/www $ npm install
... blah blah blah
../wasm-dot/www $ npm run start
> create-wasm-app@0.1.0 start /home/ben/code/wasm-dot/www
> webpack-dev-server

ℹ ｢wds｣: Project is running at http://localhost:8080/
ℹ ｢wds｣: webpack output is served from /
ℹ ｢wds｣: Content not from webpack is served from /home/ben/code/wasm-dot/www
ℹ ｢wdm｣: Hash: f1490ee1682760eb8f0e
Version: webpack 4.33.0
Time: 345ms
Built at: 05/14/2020 4:47:08 PM
...blah blah blah
```

Open your web browser to `localhost:8080` and play with your brand new dot.