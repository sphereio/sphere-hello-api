#!/bin/bash
# be verbose:
set -x
# return error for whole script if any command returns error (append " || true" to override that for a line):
set -e
# clear the fog about what's in the path:
java -version || true

# make sure groovy is there
sudo apt-get install groovy

# print groovy version for transparency. cross-check the java version
groovy -version || true
java -version || true

# our travis is running on JVM8, too, but Groovy only supports JVM8 since Groovy 2.3, which isn't available on travis.)
# the second line returns 
GROOVY_JVM=$(groovy -version 2>&1 | sed 's/.*JVM: \(.*\)\.\(.*\)\..*/\1\2/; 1q')

# if groovy JVM is max 1.7, get stuff and grep for the result
[ "$GROOVY_JVM" -ge 18 ] && echo "groovy is not compatible with JDK8 yet, skipping test" || groovy ./apidump.groovy | grep "MB PREMIUM TECH T"
