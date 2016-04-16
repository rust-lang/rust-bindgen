#!/usr/bin/env python

import os
import sys
import subprocess
import tempfile

BINDGEN_FLAGS_PREFIX = "// bindgen-flags: ";
COMMON_PRELUDE = """
#![feature(const_fn)]
"""

if len(sys.argv) != 4:
  print("Usage: {} [bindgen-path] [c-path] [rust-path]\n".format(sys.argv[0]))

flags = [];
with open(sys.argv[2]) as f:
  for line in f:
    if line.startswith(BINDGEN_FLAGS_PREFIX):
      flags = line.strip().split(BINDGEN_FLAGS_PREFIX)[1].split(' ')

base_command = [sys.argv[1], sys.argv[2], "-o", sys.argv[3]]

for line in COMMON_PRELUDE.split('\n'):
  flags.append("-raw-line")
  flags.append(line)

base_command.extend(flags);
subprocess.check_call(base_command, cwd=os.getcwd())


name = None
with tempfile.NamedTemporaryFile(delete=False) as tests:
  name = tests.name
  subprocess.check_call(["rustc", "--test", sys.argv[3], "-o", tests.name])
subprocess.check_call([tests.name])
