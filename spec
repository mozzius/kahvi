#!/bin/sh
# Look up ECMA-262 clauses. Usage: ./spec sec-array.prototype.flatmap | ./spec --find flatMap
exec python3 "$(dirname "$0")/harness/spec.py" "$@"
