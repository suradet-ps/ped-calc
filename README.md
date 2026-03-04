# PedCalc

PedCalc is a fast, client-side pediatric dosage calculator built with Rust and the Leptos WebAssembly framework. It is designed to assist healthcare professionals in calculating medication dosages quickly and accurately. The application operates completely within the browser without requiring a backend server.

**⚠️ Disclaimer**: This tool is not a substitute for clinical judgment and medical knowledge. Users must verify all calculations before prescribing or administering medication.

## Key Features

- **Age and Weight-Based Calculations**: Computes precise dosage ranges based on an infant's, child's, or adolescent's actual body weight.
- **Built-in Drug Database**: Supports 9 commonly used pediatric drugs, including Amoxicillin, Co-amoxiclav, Paracetamol, Ibuprofen, Cetirizine, Azithromycin, Oseltamivir, Co-trimoxazole, and Metronidazole.
- **Clinical Alerts & Notes**: Provides active clinical alerts based on the selected drug, covering contraindications and adjustments (e.g., renal or hepatic impairment).
- **Unit Conversions**: Automatically converts milligram (mg) dosages into milliliter (mL) volumes using default or available drug formulations.
- **History Tracking**: Automatically saves the user's calculation history locally within the browser (`localStorage`).
- **High Performance**: Highly optimized and statically typed through Rust and WebAssembly.

## Project Structure

A brief overview of the `src/` directory layout:

- `main.rs` & `app.rs`: Application entry points, context provision, and routing.
- `components/`: UI elements like `layout`, `navbar`, and `footer`.
- `pages/`: Primary application views (`calculator.rs`, `drug_reference.rs`, `about.rs`).
- `data/`: The static, in-memory drug database (`drugs.rs`) and interaction definitions (`interactions.rs`).
- `logic/`: Core clinical and arithmetic logic separated into calculation (`calculator.rs`), adjustment (`adjuster.rs`), validation (`validator.rs`), and formatting (`formatter.rs`).
- `types/`: Type definitions and structures representing patients, drugs, and computational state.

## Installation & Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2021)
- [Trunk](https://trunkrs.dev/) (WASM application bundler)
- The WebAssembly compile target (`wasm32-unknown-unknown`)

### Setup and Running Locally

1. Install the WASM target:

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Install Trunk:

   ```bash
   cargo install trunk
   ```

3. Start the development server:

   ```bash
   trunk serve
   ```

   The application will be available at [http://127.0.0.1:8080](http://127.0.0.1:8080).

### Building for Production

To create a release bundle optimized for size and speed:

```bash
trunk build --release
```

The output will be placed in the `dist/` folder.

## Deployment

PedCalc can be easily deployed to [Vercel](https://vercel.com) as a Static Site. The repository includes the necessary configuration files.

1. Create a new project on Vercel and import your repository.
2. In the **Build and Output Settings**:
   - **Framework Preset**: `Other`
   - **Build Command**: `bash build.sh`
   - **Output Directory**: `dist`
3. Click **Deploy**. Vercel will run the included `build.sh` script to install Rust, WebAssembly targets, and Trunk, and then build the application.

## Usage

1. Start the application locally and access the calculator view.
2. Input the patient's demographic details (Weight, Age).
3. Select an antibiotic, analgesic, or other medication from the built-in database dropdown.
4. Review the clinically validated dosage limits, mg-to-mL conversion based on formulations, and necessary clinical checks.
5. Review the "History" or "Drug Reference" sections as needed.

## Configuration

- `Cargo.toml`: Manages Rust dependencies (such as `leptos`, `rust_decimal` for accurate float math, and `serde`). It also outlines optimization flags to reduce the WASM bundle size.
- `Trunk.toml`: Manages development server configurations, watch targets, and build settings (like running `wasm-opt`).

## Contributing

Contributions are welcome. Please read our [Contributing Guidelines](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

The static clinical logic and dosing data references are sourced from:

- BNF for Children 2023-2024
- Harriet Lane Handbook, 22nd Edition
- Thai Pediatric Formulary 2023
- UpToDate Pediatric Drug Information
- WHO influenza treatment guidelines 2022
