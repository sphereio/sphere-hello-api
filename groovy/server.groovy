import org.eclipse.jetty.server.Server
import org.eclipse.jetty.servlet.*
import groovy.servlet.*
import groovy.json.JsonSlurper
import javax.xml.bind.DatatypeConverter

@Grapes([
    @Grab(group='org.eclipse.jetty.aggregate', module='jetty-server', version='7.6.0.v20120127'),
    @Grab(group='org.eclipse.jetty.aggregate', module='jetty-servlet', version='7.6.0.v20120127'),
    @Grab(group='javax.servlet', module='servlet-api', version='2.5')])

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

def startJetty(config) {
	def server = new Server(8080)
	def context = new ServletContextHandler(ServletContextHandler.SESSIONS)
	context.contextPath = '/'
	context.resourceBase = '.'
	context.setInitParameter("project_key", config.project_key)
	context.setInitParameter("access_token", config.access_token)
	context.addServlet(GroovyServlet, '/index.groovy')
	server.handler = context
	server.start()
}

println "Getting oauth token"
def oauth_data = getOauthToken(config)
config.access_token = oauth_data["access_token"]
println "Starting Jetty"
startJetty(config)
println "open http://localhost:8080/index.groovy | press Ctrl+C to stop the server."
