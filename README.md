# Rust Mechanical Gears App

A mechanical gear app written in Rust. It supports both command-line and
WebAssembly (WASM) outputs.

### Getting Started

1. Clone the repository.
2. Install libraries: `cargo install wasm-pack`
3. Build CLI: `rustc gear-cli.rs -o gear-cli`
4. Build WASM library: `cd gear_calculator; wasm-pack build --release`
5. Run the program `./gear-cli` or JavaScript code:
```javascript
import init, { GearTrain } from './gear_calculator/pkg/gear_calculator.js';

async function run() {
  await init();

  const train = new GearTrain(20, 60, 2.0);
  const result = train.calculate(1200);

  console.log(JSON.parse(result));
  // Output: { gear_ratio: 3, driven_rpm: 400, center_distance: 80 }
}

run();
```