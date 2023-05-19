# vmod_unleash

This is a [varnish](https://varnish-cache.org/) vmod integrating [unleash](https://www.getunleash.io/) into VCL.
The full VCL API is described in [vmod.vcc](vmod.vcc).

## Prerequisites

It's recommended that you have a basic understanding of Unleash and Varnish Cache.

You may find it helpful to take a look at the following:

- [Unleash documentation](https://docs.getunleash.io/)
- [Introduction to Varnish](https://varnish-cache.org/intro/index.html#intro)

## Installation

### From deb/rpm package

You can follow steps specified in Dockerfiles located in [pkg](pkg) directory, but instead
of using open-source Varnish version, you'll have to [install Varnish Plus packages](https://docs.varnish-software.com/varnish-enterprise/installation/).

### From Source

You'll need following dependencies:

- `cargo` (and the accompanying `rust` package)
- `python3`
- the `varnish-plus` development libraries/headers
- OpenSSL versions 1.0.1 through 1.1.1 with headers (see [rust-openssl](https://github.com/sfackler/rust-openssl))
- Clang 5.0 or greater (see [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html))

```sh
git clone https://github.com/teqm/libvmod-unleash.git
cd libvmod-unleash
cargo build --release
cp ./target/release/libvmod_unleash.so {_libdir}/varnish-plus/vmods/
```

## VCL examples

### Feature flags

```vcl
import unleash;

sub vcl_init {
    new client = unleash.client(
        url="http://localhost:4242",
        # see https://docs.getunleash.io/reference/api-tokens-and-client-keys#client-tokens
        token="*:development.abcdefghijklmnopqrstu");
}

sub vcl_recv {
    if (client.is_enabled(name="toggle")) {
        # do new, flashy thing
    } else {
        # do old, boring stuff
    }
}
```

### A/B testing

```vcl
import unleash;

sub vcl_init {
    new client = unleash.client(...);
}

sub vcl_recv {
    set req.http.x-variant = client.get_variant(name="toggle");
    
    if (req.http-x-variant == "A") {
        # variant A
    } else if (req.http-x-variant == "B") {
        # variant B
    } else {
        # variant C
    }
}
```

### Integration with backends

You can use Unleash in e.g. Node.js application and integrate it with Varnish using this VMOD. This way Varnish
doesn't have to hit backend on every request (as the response could vary per request) and
instead cache key is computed in VMOD itself and retrieved via `.get_hash()` method.

```vcl
import unleash;

sub vcl_init {
    new client = unleash.client(...);
}

sub vcl_recv {
    # this header must be included in backend's Vary header,
    # to split the cache based on a computed feature set
    set req.http.x-features = client.get_hash(
        user_id=req.http.user-id,
        session_id=req.http.session-id);
}
```
