#!/bin/bash

login() {
    local enc="$(echo -n "${CLIENT_ID}:${CLIENT_SECRET}" | base64)"
    local result="$(curl -s -X POST -H "Authorization: Basic ${enc}" -d "grant_type=client_credentials&scope=manage_project:${PROJECT_KEY}" https://auth.sphere.io/oauth/token)"
    local t="$(echo $result | jq  '.access_token')"
    local t="${t%\"}" # removing leading quote
    readonly ACCESS_TOKEN="${t#\"}" # removing trailing quote
    if [ "$ACCESS_TOKEN" = "null" ]; then
        echo "$result"
        exit 2
    fi
}
