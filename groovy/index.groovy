
import groovy.json.JsonSlurper
import javax.servlet.ServletContext


def callSphere(String endpoint, String query){
	def ServletContext app = application
	def project_key = app.getInitParameter("project_key")
	def access_token = app.getInitParameter("access_token")

	def apiUrl = new URL("https://api.sphere.io/$project_key/$endpoint?$query")
	def HttpURLConnection connection = apiUrl.openConnection()
	connection.setRequestMethod("GET")
	connection.setRequestProperty("Authorization", "Bearer $access_token")
	connection.setDoInput(true)
	connection.connect()
	def result = new JsonSlurper().parse(new InputStreamReader(connection.getInputStream(),"UTF-8"))
	connection.disconnect()
	return result
}

def productProjections = callSphere("product-projections", "")

// using the "html" autobound groovlet magic to create the output. See http://groovy.codehaus.org/Groovlets, esp. the MarkupBuilder
// TIP: use the ?. "safe dereference" Groovy operator when navigating optional parts of the API response, e.g. in localized string maps

html.html {
	head {
		title 'SPHERE.IO Groovy Hello World'
	}
	body {
		h1 'Welcome to SPHERE.IO via Groovy'
		p "Found the following ${productProjections.count} Products"
		ul {
			for ( product in productProjections?.results) {
				li{
					p "${product.name?.en}"
					for(image in product.masterVariant.images){
						img(src : image.url, alt : product.name?.en, width: "150px", height: "150px")
					}
				}
			}
		}
	}
}
