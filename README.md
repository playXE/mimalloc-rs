# mimalloc-rs

[mimalloc](https://github.com/microsoft/mimalloc/) bindings for Rust.

Using as global allocator:

```rust
#[global_allocator]
static A: mimalloc_rs::MiMalloc = mimalloc_rs::MiMalloc;


fn main() {
    let mut v = Vec::with_capacity(256);

    for _ in 0..100 {
        v.push(String::from("wow"));
    }
    v.iter().for_each(|x| println!("{}",x));
}

```