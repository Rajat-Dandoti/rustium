#!/bin/bash

cargo test --test functional_tests
cargo test --test performance_tests
cargo test --test security_tests
cargo test --test accessibility_tests