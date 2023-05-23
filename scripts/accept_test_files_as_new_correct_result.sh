#!/usr/bin/env bash

# Simple script to accept the generated test results as the new correct result.
# This saves a good amount of manual work when dealing with multiple failing tests.

rename test net ./examples/results/**/**/test.*
