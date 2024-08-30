# Persian Date (Rust)

Persian Date for Rust based on chrono date-time library

[![crates.io](https://img.shields.io/crates/v/persian_date.svg)](https://crates.io/crates/persian_date) [![Documentation](https://img.shields.io/badge/Docs-persian_date-blue.svg)](https://docs.rs/persian_date/latest/persian_date)

## Installation

Add this dependency in your Cargo.toml file

```toml
[dependencies]
persian_date = "1.0.1"
```

## Usage

```rust
use persian_date::structure::PDate;

let pdate = PDate::now(); // initialize date
println!("{}",pdate); // formatted date
println!("{}",pdate.year()); // jalali year
println!("{}",pdate.month()); // jalali month
println!("{}",pdate.day()); // jalali day
println!("{}",pdate.day_of_week()); // day of week from saturday
// and more ...
```
Full documentation in [docs.ir](https://docs.rs/persian_date/latest/persian_date/index.html)

## Formatting

Date formatting in this library is similar to [chrono](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)

```rust
pdate.format("%Y-%m-%d %H:%M:%S") 
```