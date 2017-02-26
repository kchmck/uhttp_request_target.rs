//! This crate provides a parser for classifying an HTTP [request line
//! target](https://tools.ietf.org/html/rfc7230#section-5.3) into one of the 4 types
//! defined for requests. This can then be used to direct how to further process the
//! target.
//!
//! ## Examples
//!
//! ```rust
//! use uhttp_request_target::RequestTarget;
//!
//! assert_eq!("/r/rust".parse(), Ok(RequestTarget::AbsPath));
//! assert_eq!("https://example.com".parse(), Ok(RequestTarget::AbsURI));
//! assert_eq!("example.com".parse(), Ok(RequestTarget::Authority));
//! assert_eq!("*".parse(), Ok(RequestTarget::ServerOptions));
//! ```

/// A request target that appears in every HTTP request start line.
///
/// This gives a hint as to how the target should be interpreted but doesn't guarantee the
/// matched string has well-formed syntax.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RequestTarget {
    /// General form used for direct requests targeting a resource on the origin server.
    AbsPath,
    /// Currently only used with the proxy protocol, but HTTP/1.1 servers must accept this
    /// form for other requests too.
    AbsURI,
    /// Used with CONNECT in the proxy protocol.
    Authority,
    /// Used for server-wide OPTIONS request.
    ServerOptions,
}

impl std::str::FromStr for RequestTarget {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::RequestTarget::*;

        // Surrounding whitespace and empty string are invalid [RFC7230§3.1.1,
        // RFC7230§5.3].
        if s != s.trim() || s.is_empty() {
            return Err(());
        }

        if s == "*" {
            // The OPTIONS form contains only an asterisk [RFC7230§5.3.4].
            Ok(ServerOptions)
        } else if s.starts_with('/') {
            // The absolute path form always starts with a slash [RFC7230§5.3.1].
            Ok(AbsPath)
        } else if s.starts_with("http://") || s.starts_with("https://") {
            // The URI form starts with one of the two HTTP schemes [RFC7230§5.3.2].
            Ok(AbsURI)
        } else if !s.contains('/') {
            // The authority form contains no slashes [RFC7230§5.3.3].
            Ok(Authority)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_request_target() {
        use self::RequestTarget::*;

        assert_eq!("".parse::<RequestTarget>(), Err(()));
        assert_eq!("  ".parse::<RequestTarget>(), Err(()));
        assert_eq!("\t\n\r\u{2008}\u{00A0}\u{205F}".parse::<RequestTarget>(), Err(()));
        assert_eq!("".parse::<RequestTarget>(), Err(()));

        assert_eq!("*".parse::<RequestTarget>(), Ok(ServerOptions));
        assert_eq!(" *".parse::<RequestTarget>(), Err(()));
        assert_eq!("* ".parse::<RequestTarget>(), Err(()));
        assert_eq!("   *  ".parse::<RequestTarget>(), Err(()));

        assert_eq!("/path/sub/42".parse::<RequestTarget>(), Ok(AbsPath));
        assert_eq!("/path/sub/42?key=value".parse::<RequestTarget>(), Ok(AbsPath));
        assert_eq!("/where?q=now".parse::<RequestTarget>(), Ok(AbsPath));
        assert_eq!(" /path/sub/42".parse::<RequestTarget>(), Err(()));
        assert_eq!("/path/sub boop/42".parse::<RequestTarget>(), Ok(AbsPath));

        assert_eq!("www.example.com:80".parse::<RequestTarget>(), Ok(Authority));
        assert_eq!("www.example.com".parse::<RequestTarget>(), Ok(Authority));
        assert_eq!("example.com".parse::<RequestTarget>(), Ok(Authority));
        assert_eq!("user@example.com".parse::<RequestTarget>(), Ok(Authority));
        assert_eq!("user@example.com/".parse::<RequestTarget>(), Err(()));
        assert_eq!("user name@example.com".parse::<RequestTarget>(), Ok(Authority));

        assert_eq!("http://zombo.com".parse::<RequestTarget>(), Ok(AbsURI));
        assert_eq!("http://picard.ytmnd.com/".parse::<RequestTarget>(), Ok(AbsURI));
        assert_eq!("https://rust-lang.org".parse::<RequestTarget>(), Ok(AbsURI));
        assert_eq!("https://rust-lang.org/a path".parse::<RequestTarget>(), Ok(AbsURI));
        assert_eq!("http:/zombo.com".parse::<RequestTarget>(), Err(()));
        assert_eq!("file:/rust-lang.org".parse::<RequestTarget>(), Err(()));
        assert_eq!("ftp://rust-lang.org".parse::<RequestTarget>(), Err(()));
    }
}
