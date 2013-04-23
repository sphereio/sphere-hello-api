require 'base64'
require 'excon'
require 'json'

def login(client_id, client_secret, project_key)
  encoded = Base64.urlsafe_encode64 "#{client_id}:#{client_secret}"
  headers = { 'Authorization' => "Basic #{encoded}", 'Content-Type' => 'application/x-www-form-urlencoded' }
  body = "grant_type=client_credentials&scope=manage_project:#{project_key}"
  res = Excon.post 'https://auth.sphere.io/oauth/token', :headers => headers, :body => body
  raise "Problems on getting access token from auth.sphere.io: #{res.body}" unless res.status == 200
  JSON.parse(res.body)['access_token']
end
