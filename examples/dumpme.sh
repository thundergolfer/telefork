#!/usr/bin/env bash
#
# Trivial bash script that can be dumped with dump.rs and restored
# with load.rs.
#
# It does restore successfully (nice!) but the script crashes on teardown
# with
#
# ./examples/dumpme.sh: error reading input file: Bad file descriptor
#
# I think this is because telefork isn't restoring any file descriptors.

for i in {1..20}
do
  echo "step $i"
  sleep 1
done
