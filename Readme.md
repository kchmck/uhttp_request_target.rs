# uhttp\_request\_target -- HTTP request target parser

[Documentation](https://docs.rs/uhttp_request_target)

This crate provides a parser for classifying an HTTP [request line
target](https://tools.ietf.org/html/rfc7230#section-5.3) into one of the 4 types
defined for requests. This can then be used to direct how to further process the
target.

## Examples

```rust
use uhttp_request_target::RequestTarget;

assert_eq!("/r/rust".parse(), Ok(RequestTarget::AbsPath));
assert_eq!("https://example.com".parse(), Ok(RequestTarget::AbsURI));
assert_eq!("example.com".parse(), Ok(RequestTarget::Authority));
assert_eq!("*".parse(), Ok(RequestTarget::ServerOptions));
```

## Usage

This [crate](https://crates.io/crates/uhttp_request_target) can be used through cargo by
adding it as a dependency in `Cargo.toml`:

```toml
[dependencies]
uhttp_request_target = "0.5.0"
```
and importing it in the crate root:

```rust
extern crate uhttp_request_target;
```
