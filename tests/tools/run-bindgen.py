#!/usr/bin/env python

import os
import sys
import subprocess
import tempfile

BINDGEN_FLAGS_PREFIX = "// bindgen-flags: ";
COMMON_PRELUDE = """
#![allow(non_snake_case)]
"""

if len(sys.argv) != 4:
  print("Usage: {} [bindgen-path] [c-path] [rust-path]\n".format(sys.argv[0]))

flags = [];
with open(sys.argv[2]) as f:
  for line in f:
    if line.startswith(BINDGEN_FLAGS_PREFIX):
      flags = line.strip().split(BINDGEN_FLAGS_PREFIX)[1].split(' ')

base_command = [sys.argv[1], "-o", sys.argv[3]]

for line in COMMON_PRELUDE.split('\n'):
  flags.append("-raw-line")
  flags.append(line)

base_command.extend(flags);
base_command.append(sys.argv[2]);
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
