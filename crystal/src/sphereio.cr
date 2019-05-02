require "halite"
require "base64"

module SphereIO
  def self.login(client_id, client_secret, project_key)
    encoded = Base64.urlsafe_encode "#{client_id}:#{client_secret}"
    headers = {
      "Authorization" => "Basic #{encoded}",
      "Content-Type"  => "application/x-www-form-urlencoded",
    }

    body = "grant_type=client_credentials&scope=manage_project:#{project_key}"
    resp = Halite.post("https://auth.sphere.io/oauth/token", headers: headers, raw: body)

    # Raise error on bad return status
    resp.raise_for_status

    # Return our access token as a string
    resp.parse("json")["access_token"].as_s
  rescue ex : Halite::ClientError | Halite::ServerError
    raise "Problems on getting access token from auth.sphere.io: [#{ex.status_code}] #{ex.status_message} (#{ex.class})"
  rescue ex
    raise "Problems on getting access token from auth.sphere.io: #{ex.message}"
  end
end
