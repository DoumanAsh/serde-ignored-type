# serde_ignored_type

[![CI](https://github.com/DoumanAsh/serde_ignored_type/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/serde_ignored_type/actions)
[![Crates.io](https://img.shields.io/crates/v/serde_ignored_type.svg)](https://crates.io/crates/serde_ignored_type)
[![Documentation](https://docs.rs/serde_ignored_type/badge.svg)](https://docs.rs/crate/serde_ignored_type/)

Simple utility type to allow to skip value for purpose of deserialization.

Why would you want to use it?

Because sometimes you just do not care for value of map, and need key alone

Unfortunately serde interface is too dumb to just skip next value
