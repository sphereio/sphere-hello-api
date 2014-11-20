#!/bin/bash

cat > "config.js" << EOF
/* SPHERE.IO credentials */
exports.config = {
  client_id: "${NODE_SPHERE_CLIENT_ID}",
  client_secret: "${NODE_SPHERE_CLIENT_SECRET}",
  project_key: "${NODE_SPHERE_PROJECT_KEY}"
}
EOF