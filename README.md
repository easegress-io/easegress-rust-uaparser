# Easegress-rust-uaparser

Easegress-rust-uaparser is a user-agent parser for the Easegress and Cloudflare platforms. It is inspired by the  [uap-core](https://github.com/ua-parser/uap-core) and [user-agent-parser](https://github.com/magiclen/user-agent-parser) projects.

## Usage

To use the parser, follow these steps:

### Using Easegress's WASM

   - Navigate to the `binary` directory.
   - Use `easegress.wasm` in your Easegress configuration file.
  
   or you can build the parser yourself:

   - Navigate to the `easegress` directory.
   - (Optional) Edit the `regexes.yaml` file as needed to customize the parser's behavior.
   - Add the `wasm32-unknown-unknown` target to your Rust installation using the command `rustup target add wasm32-unknown-unknown`.
   - Build the parser into a WebAssembly module using the command `cargo build --target wasm32-unknown-unknown --release`. If the build is successful, the resulting `easegress.wasm` file will be located in the `target/wasm32-unknown-unknown/release` directory.

### Using Cloudflare's Workers

   - Navigate to the `cloudflare` directory.
   - Edit the `wrangler.toml` file to configure your deployment settings.
   - (Optional) Edit the `regexes.yaml` file as needed to customize the parser's behavior.
   - Publish the worker to Cloudflare using the command `npx wrangler publish`.
   - Once the worker is deployed, you can add Workers Routes to your website to enable the worker to process incoming user-agent strings.

The parser is designed to extract device and OS information from user-agent strings, which are sent by web browsers and other HTTP clients to identify themselves to servers. Specifically, the parser extracts this information and sets `x-ua-device` and `x-ua-header` headers in the request. 

Note that this snippet assumes some prior knowledge of Rust, WebAssembly, and Cloudflare. If you're not familiar with these technologies, you may need to do some additional research to understand how to use the parser.
