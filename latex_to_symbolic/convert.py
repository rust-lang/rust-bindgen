#!/usr/bin/env python

#This script is a mitigation for some bugs in rust-bindgen

import os
with open(os.path.join(os.getenv("OUT_DIR"), "bindings.rs")) as infile:
	with open("modbind.rs", "w") as outfile:
		for line in infile:
			if line not in ["pub type pointer = pointer;\n",
							"pub type const_pointer = const_pointer;\n",
							"pub type const_pointer = *const GiNaC_ex;\n",
							"pub type pointer = *mut GiNaC_expair;\n",
							"    pub static mut GiNaC_first: *mut GiNaC_class_info<OPT>;\n",
							"    #[link_name = \"\\u{1}first\"]\n"]:
				outfile.write(line)