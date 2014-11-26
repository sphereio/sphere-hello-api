#!/bin/bash

cat > "java/config.properties" << EOF
projectKey = $PROJECT_KEY
clientId = $CLIENT_ID
clientSecret = $CLIENT_SECRET
EOF

cat > "curl/config" << EOF
CLIENT_ID="$CLIENT_ID"
CLIENT_SECRET="$CLIENT_SECRET"
PROJECT_KEY="$PROJECT_KEY"
EOF

cat > "python2/config.py" << EOF
PROJECT_KEY = "$PROJECT_KEY"
CLIENT_ID = "$CLIENT_ID"
CLIENT_SECRET = "$CLIENT_SECRET"
EOF

cat > "python3/config.py" << EOF
PROJECT_KEY = "$PROJECT_KEY"
CLIENT_ID = "$CLIENT_ID"
CLIENT_SECRET = "$CLIENT_SECRET"
EOF