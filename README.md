# ffi-opaque

A macro generating correct opaque types.

## What it does

Until [RFC 1861 (Extern types)](https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md) is implemented, representing opaque structs in Rust is hard, as decribed in the [corresponding Nomicon section](https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs).

Yet, opaque structs are a common pattern, especially when working with C codebases through FFI.

`ffi-opaque` provides the `opaque!` macro which properly generates such types without having to think about the details.

Types generated by this macro are:
* are zero-sized
* cannot be constructed outside of the module they are defined in
* have proper alignment
* are `!Send`, `!Sync`, `!Unpin`
* are FFI safe

## Usage example

Consider this example from the `leveldb` C API:

```c
typedef struct leveldb_options_t leveldb_options_t;
typedef struct leveldb_writeoptions_t leveldb_writeoptions_t;

leveldb_options_t* leveldb_options_create();
leveldb_writeoptions_t* leveldb_writeoptions_create();
```

It uses an opaque struct to avoid leaking structural details of its database options to a library linking to it. We can represent the opaque structs on the Rust side like this:

```rust
use ffi_opaque::opaque;

opaque! {
    /// Documentation works!
    pub struct leveldb_options_t;
    /// Options used when writing data
    pub struct leveldb_writeoptions_t;
}

extern "C" {
    pub fn leveldb_options_create() -> *mut leveldb_options_t;
    pub fn leveldb_writeoptions_create() -> *mut leveldb_writeoptions_t;
}
```

## Possible future extensions

If `extern` types become stabilised, this macro may adopt them.

## License

MIT, see LICENSE file.

## Acklowledgements

* [David Tolnay (@dtolnay)](https://github.com/dtolnay) for raising awareness of many subtle issues
* [Nikolai Vasquez (@nvzqz)](https://github.com/nvzqz) for implementing the final type and a number of checks