#!/bin/bash

cargo run

lli out.bc

echo "Result: $?"
