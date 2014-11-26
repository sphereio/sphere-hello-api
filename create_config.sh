#!/bin/bash

cat > "java/config.properties" << EOF
projectKey = $PROJECT_KEY
clientId = $CLIENT_ID
clientSecret = $CLIENT_SECRET
EOF

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