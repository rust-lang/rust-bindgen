#!/usr/bin/bash

#gcc -c example.cpp || exit 1
clang++ -c example.cpp || exit 1
export OUT_DIR="/home/volker/Sync/git/rust-bindgen/safe_wrappers/target/debug/build/safe_wrappers-a406ac0df99c2295/out"
/home/volker/Sync/git/rust-bindgen/target/release/bindgen example.cpp --gen-safe-wrappers -- -x c++ > out.rs || exit 0
rustc out.rs --crate-type lib || exit 0
#--emit=dep-info,metadata 
echo "finished"
exit 1



#   DIR=`mktemp -d`
#   cp /home/volker/Sync/git/rust-bindgen/safe_wrappers/example.cpp $DIR
#   cd $DIR
#   /home/volker/Sync/git/rust-bindgen/safe_wrappers/test.sh
#   echo $?
