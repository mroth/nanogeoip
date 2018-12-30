# nanogeoip :dragon:

[![Build Status](https://travis-ci.org/mroth/nanogeoip.svg?branch=master)](https://travis-ci.org/mroth/nanogeoip)

<!--
[![Docker Build](https://img.shields.io/docker/build/mrothy/nanogeoip.svg)](https://hub.docker.com/r/mrothy/nanogeoip)
[![RustDocs](https://docs.rs/nanogeoip/badge.svg)](https://docs.rs/nanogeoip)
-->

A tiny and blazing fast HTTP based microservice for extremely minimal geoip
location lookups.

This is a work-in-progress experimental Rust port of [tinygeoip] (by the same
author). It is additionally built on top of rapidly evolving unstable crates and
language features, so should not be used in production just quite yet.

<!-- It bundles into a ~2MB docker image that can serve over ~800K reqs/sec
(uncached). -->

[tinygeoip]: https://github.com/mroth/tinygeoip

## API

Identical to [tinygeoip], so for now, go there for documentation.

## Performance

TODO

## Installing and running the server

Compile with standard Rust toolchain or download a binary for your platform from
the [Releases] page.

```
nanogeoip 0.1.0
A tiny (and blazing fast!) geoip lookup microservice

USAGE:
    nanogeoip [OPTIONS] [db]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a <addr>          Address to listen for connections on [default: 0.0.0.0]
    -o <origin>        'Access-Control-Allow-Origin' header [default: *]
    -p <port>          Port to listen for connections on [default: 9000]

ARGS:
    <db>    MaxMind database file [default: data/GeoLite2-City.mmdb]

```

[releases]: https://github.com/mroth/tinygeoip/releases

## Rust library

For more information, see the [Rust Docs]. (Note: these are only built online
for tagged and pushed releases on crates.io, so you'll have to build locally if
you need them right now.)

[rust docs]: https://docs.rs/nanogeoip

<!-- ## Docker Image

A docker image is automatically built from all tagged releases.

To utilize it, be sure to mount your MaxMindDB database as a volume so that the
running container can access it.

_[TODO: provide an example for folks not so familiar with Docker.]_ -->

## Stability

:construction: The current API is considered _unstable_. This is just being
released and I'd like some feedback to make any potential changes before tagging
a `v1.0` which will maintain API stability.

In other words, comments and feedback wanted!

## License

This is still in progress and is not yet considered released software yet.
Please contact me if you have a need for a license ASAP.

## Code of Conduct

Please note that this project is released with a [Contributor Code of
Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to
abide by its terms.
