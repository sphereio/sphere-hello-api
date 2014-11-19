#!/bin/bash

cat > "java/config.properties" << EOF
projectKey = $PROJECT_KEY
clientId = $CLIENT_ID
clientSecret = $CLIENT_SECRET
EOF

cat > "python/config.py" << EOF
PROJECT_KEY = "$PROJECT_KEY"
CLIENT_ID = "$CLIENT_ID"
CLIENT_SECRET = "$CLIENT_SECRET"
EOF