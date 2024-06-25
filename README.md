# Simple Mocking example

### Clone repo:
```
git clone https://github.com/bragaigor/mocking-example.git
cd mocking-example
```

### Running the production code:
```
cargo run --release
```
Here you should see `"Hello there this is the real implementation using unicorn pups: ..."` to stdout.

### Testing mocked version:
```
cargo test --package mocking --bin mocking -- tests::unicorn_tests::tests::test_start_something_mocked --exact --show-output 
```
Here on the other hand you should see the mocked version of `start()` which in turn you should see `"This is the mock version of start!!!""` to stdout instead.
