# Sorting Library on Rust

## Description
This library provides various sorting algorithms implemented in Rust.

## Features

- Quick Sort
- Selection Sort
- Insertion Sort
- Merge Sort

## Usage

To use this library, add the following to your `Cargo.toml`:

```toml
[dependencies]
rust-sorting-library = "0.1.0"

use [rust_sorting_library]::sorting;

let mut numbers = vec![4, 2, 1, 3];
sorting::quick_sort(&mut numbers);

Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

License

MIT

```