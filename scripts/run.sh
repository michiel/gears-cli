#!/bin/sh

RUST_BACKTRACE=1 RUST_LOG=xflow::validation=debug ./target/debug/gears-cli \
  --locale nl_NL \
  --path ../xflow-rust/resource/projects/random \
  export 
