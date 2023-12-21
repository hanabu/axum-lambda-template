# Axum, lambda-http integration sample

I previously wrote the [lambda\_web](https://crates.io/crates/lambda-web) crate, but recent [axum](https://crates.io/crates/axum) and [lambda\_http](https://crates.io/crates/lambda_http) can be integrated directly.

This repository is a sample that uses axum and lambda-http. By separating the generation of `axum::Router` into a library crate, it can run on Lambda, a regular server, or integration tests. I recommend such implementation as it makes your code portable.

This sample is based on axum=0.7, hyper=1, lambda\_http=0.9 combination. If you need older axum=0.6 sample, you can find it on [axum06](https://github.com/hanabu/axum-lambda-template/tree/axum06) branch.

