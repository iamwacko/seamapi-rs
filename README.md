## Seam for Rust
[![dependency status](https://deps.rs/crate/seamapi-rs/0.1.1/status.svg)](https://deps.rs/crate/seamapi-rs/0.6.2)

Control locks, lights, and other internet of things devices with Seam's simple API. Checkout out the [documentation](https://docs.getseam.com) or [some examples](examples).

## Setup

```bash
cargo add seamapi-rs
```

## Usage

```rust
use seamapi_rs::Seam;

fn main() {
	let seam = Seam::new(None, None).expect("Failed to get key or URL");

	let workspace = seam.workspaces().get().expect("Failed to get");

	println!("{:?}", workspace);
}
```

## Development

This project is written in Rust, so use the latest stable from [Rustup](https://rustup.rs/).

- To run tests, run `cargo test`
- To build for use `cargo build --release`

Our tests use a seam sandbox environment given by environment variable `SEAM_SANDbOX_API_KEY`. If you want to run the tests, you should first create a sandobx workspace [on your dashboard](https://dashboard.getseam.com).
