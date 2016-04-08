#!/bin/bash

cat > "scala/src/main/resources/application.conf" << EOF
sphere-hello {
  clientId=$CLIENT_ID
  clientSecret=$CLIENT_SECRET
  projectKey=$PROJECT_KEY
}
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

cat > "groovy/config.groovy" << EOF
project_key = "$PROJECT_KEY"
client_id = "$CLIENT_ID"
client_secret = "$CLIENT_SECRET"
EOF
