#!/usr/bin/env python

import os
import sys
import subprocess
import tempfile

BINDGEN_FLAGS_PREFIX = "// bindgen-flags: ";
CLANG_FLAGS_SEPARATOR = "-- "
COMMON_PRELUDE = """
#![allow(non_snake_case)]
"""

if len(sys.argv) != 4:
  print("Usage: {} [bindgen-path] [c-path] [rust-path]\n".format(sys.argv[0]))
  sys.exit(1)

[_, bindgen_path, c_path, rust_path] = sys.argv

flags = ["--no-unstable-rust"]

with open(sys.argv[2]) as f:
  for line in f:
    if line.startswith(BINDGEN_FLAGS_PREFIX):
      flags.extend(line.strip().split(BINDGEN_FLAGS_PREFIX)[1].split(" "))
      break

base_command = [bindgen_path, "-o", rust_path]

for line in COMMON_PRELUDE.split("\n"):
  base_command.append("--raw-line")
  base_command.append(line)

base_command.extend(flags)
base_command.append(c_path)

env = os.environ.copy()

# El Capitan likes to unset dyld variables
# https://forums.developer.apple.com/thread/9233
if "DYLD_LIBRARY_PATH" not in env and "LIBCLANG_PATH" in env:
    env["DYLD_LIBRARY_PATH"] = env["LIBCLANG_PATH"]

subprocess.check_call(base_command, cwd=os.getcwd(), env=env)


name = None
with tempfile.NamedTemporaryFile(delete=False) as tests:
  name = tests.name
  subprocess.check_call(["rustc", "--test", sys.argv[3], "-o", tests.name])
subprocess.check_call([tests.name])
