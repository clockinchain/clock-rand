# Clock-Rand WASM Example

This example demonstrates how to use the `clock-rand` crate in WebAssembly applications.

## Building

1. Install `wasm-pack`:
   ```bash
   cargo install wasm-pack
   ```

2. Build the WASM package:
   ```bash
   wasm-pack build --target web
   ```

3. Serve the example:
   ```bash
   # Using a simple HTTP server
   python3 -m http.server 8000

   # Or using any static file server
   ```

4. Open `index.html` in your browser.

## Features Demonstrated

- **Fast RNG**: Xoshiro256+ for high-performance random number generation
- **Crypto RNG**: ChaCha20 for cryptographically secure random numbers
- **Blockchain RNG**: ChainSeed-X for deterministic blockchain-aware randomness
- **Performance Benchmarking**: Measure RNG performance in the browser

## Browser Compatibility

This example works in all modern browsers that support WebAssembly and ES6 modules.

## Security Note

Remember that client-side random number generation may not be suitable for all cryptographic applications. For server-side applications, use the native Rust crate instead.