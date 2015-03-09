import groovy.json.JsonSlurper
import javax.xml.bind.DatatypeConverter

def config = new ConfigSlurper().parse(new File('config.groovy').toURI().toURL())

def getOauthToken(config) {

	// Prepare oauth request
	def auth_url = new URL("https://auth.sphere.io/oauth/token")
	def query = "grant_type=client_credentials&scope=manage_project:$config.project_key"
	def secret = DatatypeConverter.printBase64Binary("$config.client_id:$config.client_secret".getBytes());

	// do the oauth HTTP request the built-in (verbose) java way:
	def HttpURLConnection connection = auth_url.openConnection()
	connection.setRequestMethod("POST")
	connection.setRequestProperty("Authorization", "Basic $secret")
	connection.setRequestProperty("Content-Type", "application/x-www-form-urlencoded")
	connection.setDoOutput(true)

	def os = connection.getOutputStream()
	os.write(query.getBytes("UTF-8"))
	os.flush()
	os.close()
	connection.connect()
	def oauth_data = new JsonSlurper().parse(new InputStreamReader(connection.getInputStream(), "UTF-8"))
	connection.disconnect()

	return oauth_data
}

def callSphere(config, String endpoint, String query){

	def apiUrl = new URL("https://api.sphere.io/$config.project_key/$endpoint?$config.query")
	def HttpURLConnection connection = apiUrl.openConnection()
	connection.setRequestMethod("GET")
	connection.setRequestProperty("Authorization", "Bearer $config.access_token")
	connection.setDoInput(true)
	connection.connect()
	def result = new JsonSlurper().parse(new InputStreamReader(connection.getInputStream(),"UTF-8"))
	connection.disconnect()
	return result
}

println "Getting oauth token"
def oauth_data = getOauthToken(config)
config.access_token = oauth_data["access_token"]

println "querying SPHERE.IO"
def productProjections = callSphere(config, "product-projections", "")

// Dump the object serialization to stdout (here is a good point to start doing something sensible)
// TODO pretty print the object
println "here are your products as a groovy friednly nested map"
println productProjections
