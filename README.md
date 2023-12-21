# Axum, lambda\_http integration sample

I previously wrote the [lambda\_web](https://crates.io/crates/lambda-web) crate, but recent [axum](https://crates.io/crates/axum) and [lambda\_http](https://crates.io/crates/lambda_http) can be integrated directly.

This repository is a sample that uses axum and lambda-http. By separating the generation of `axum::Router` into a library crate, it can run on Lambda, a regular server, or integration tests. I recommend such implementation as it makes your code portable.

This branch contains samples based on axum=0.6, hyper=0.14, lambda-http=0.8 combination. Newer versions can be found on [main](https://github.com/hanabu/axum-lambda-template/) branch.

