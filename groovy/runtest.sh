#!/bin/bash
# be verbose:
set -x
# return error for whole script if any command returns error (using || true to override below):
set -e

sudo apt-get install groovy || true

# manually prefetch dependencies to make startup time of the script more predictable
grape install org.eclipse.jetty.aggregate jetty-server 7.6.0.v20120127
grape install org.eclipse.jetty.aggregate jetty-servlet 7.6.0.v20120127
grape install javax.servlet servlet-api 2.5

chmod +x ./server.groovy

# start the server (do not use nohup because we want the return code in $! and the output visible)
groovy ./server.groovy &
# have to wait a while because on first start a jar needs to be downloaded
sleep 20
wget -q -O- http://localhost:8080/index.groovy | grep "MB PREMIUM TECH T"
kill $! || true
