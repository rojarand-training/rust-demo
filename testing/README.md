When creating library with `cargo new libraryname --lib`, src/lib.rs file is created with test example. 

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

[Integration tests](https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#the-tests-directory)

