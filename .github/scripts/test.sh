#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'order-test' ]; then
    cd $PROJECT
    forc build --path ../order-predicate
    forc build --path ../order-script
    forc build --path ../order-settle-contract
    forc build --path ../order-test

    cargo test
fi