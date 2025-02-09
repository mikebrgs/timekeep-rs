# Timekeep-rs

Timekeep-rs is a Rust library designed to provide efficient and reliable timekeeping functionalities. This library is built with performance and ease of use in mind, making it suitable for a wide range of applications.

## Features

- High precision timekeeping
- Easy to use API
- Lightweight and efficient
- Suitable for embedded systems

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
timekeep-rs = "0.1.0"
```

## Usage

Here is a simple example to get you started:

```rust
use timekeep_rs::{AtomicInterval, IntervalSet};

fn main() {
    let atomic_interval = AtomicInterval::closed(1, 5);
    let interval = IntervalSet::from(atomic_interval);

    println!("Interval: {}", interval.to_string());
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## Contact

For any inquiries, please contact Miguel Borges at [miguel.borges@hotmail.com].
