#!/bin/bash
# be verbose:
set -x
# return error for whole script if any command returns error (append " || true" to override that for a line):
set -e
# make sure groovy is there
sudo apt-get install groovy 
# get stuff and grep for the result
groovy ./apidump.groovy | grep "MB PREMIUM TECH T"
