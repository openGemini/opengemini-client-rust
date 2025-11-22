#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
# set -x

PORT_LI="8086 8087"

stop() {
  for i in $PORT_LI; do
    docker stop opengemini-rust-client-test-$i || true
  done
}

trap stop EXIT

boot() {
  for i in $PORT_LI; do
    docker run --rm -d -p $i:8086 --name opengemini-rust-client-test-$i opengeminidb/opengemini-server:latest || stop
  done
  for i in $PORT_LI; do
    while ! nc -z localhost $i; do
      echo "wait for port $i"
      sleep 1
    done
  done
}

boot
sleep 3

RUST_LOG=debug RUST_BACKTRACE=1 cargo test --all-features -- --nocapture
