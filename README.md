# Rust custom test framework

## Features
- Simple Unit Tests:
```rust
    use rust_unit::{test, Result};
    
    #[test]
    fn it_works() -> Result {
        try {
            assert_eq!(1, 1);
        }
    }
```
- Data Sources: 
```rust
    use rust_unit::{test, Result};
    
    fn data_source() -> impl Iterator<Item = (i32, i32, bool)> {
        vec![
            (1, 1, true),
            (1, 2, false),
            (0, 0, true),
            (-1, -1, true),
            (-1, -2, false),
        ]
        .into_iter()
    }

    #[test("{} equals {} is {}", a, b, res)]
    #[source(data_source)]
    fn it_works(a: i32, b: i32, res: bool) -> Result {
        try {
            assert_eq!(a == b, res);
        }
    }
```
## Todo
- Test Tags (Groups):
```rust
    use rust_unit::{test, Result};
    
    #[test]
    #[tags("fast", "trivial", "whatever-else")]
    fn it_works() -> Result {
        try {
            assert_eq!(1, 1);
        }
    }
```
- Custom Assertions using rust_unit::Result:
```rust
    use rust_unit::{test, Result};
    
    #[test]
    #[tags("fast", "trivial", "whatever-else")]
    fn it_works() -> Result {
        try {
            assert_eq!(1, res)?;
            assert_ne!(1, res)?;
        }
    }
```
- Actually running the tests.
- Multithreaded execution.
- Tracking source packages/span