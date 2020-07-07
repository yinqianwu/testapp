# testapp

A blazing fast web built with Vue.js (frontend) and Rocket.rs (backend).

## Pre-requisites
- Rust with nightly toolchain.
- Node.js (recommend version >= 12.x.x)

## Installing all dependencies
- Frontend:
```sh
cd web
npm install
```

- Backend:
```sh
cd ..
cargo build
```

## Running

### Development mode
If you want to customize your site or add new features use both commands (on separated terminals):
```sh
cd web
npm run dev
```
```sh
cd ..
cargo run
```
And then you can start adding new stuff or editing what already exists!

### Production mode
Running on production mode lets you generate the client files and run the client and server only using Rocket.rs.
```sh
cd web
npm run generate

cd ..
cargo run
```

## Configuration
1. On the files:
```
pages/index.vue
pages/article/_slug.vue
```

You can see that it has the string `http://localhost:8000/` it is the server address to search for the post images.

2. On the file `nuxt.config.js`, you can see:
```javascript
proxy: {
    '/api/': {
        target: 'http://localhost:8000/api/',
        pathRewrite: {
        '^/api/': ''
        },
        changeOrigin: true
    }
}
```
You will need to the same as for the above, change the server adress to the correct adress based on the rust server.

