import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.{Future, Await}
import scala.concurrent.duration.Duration
import com.typesafe.config._
import com.ning.http.client.AsyncHttpClient
import com.ning.http.util.Base64
import dispatch._
import net.liftweb.json._

object SphereHello extends App {
  val ApiUrl = "https://api.sphere.io/"
  val AuthApiUrl = "https://auth.sphere.io/oauth/token"

  //see configuration file scala/src/main/resources/application.conf
  val conf = ConfigFactory.load()
  val clientId = conf.getString("sphere-hello.clientId")
  val clientSecret = conf.getString("sphere-hello.clientSecret")
  val projectKey = conf.getString("sphere-hello.projectKey")
  
  val h = new Http(new AsyncHttpClient)
  
  /** @return oauth access token */
  def login(clientId: String, clientSecret: String, projectKey: String): Future[String] = {
    val encoded = Base64.encode(s"${clientId}:${clientSecret}".getBytes)
    h(url(AuthApiUrl).POST
      .setHeader("Authorization", s"Basic ${encoded}")
      .setHeader("Content-Type", "application/x-www-form-urlencoded")
      .setBody(s"grant_type=client_credentials&scope=manage_project:${projectKey}")
    ).map(rsp => parse(rsp.getResponseBody) \ "access_token" match {
      case JString(s) => s
      case _ => throw new IllegalStateException("Auth response does not contain 'access_token'.\n" + rsp.getResponseBody)
    })
  }
  
  /** Issues a query to get all products, see [[http://sphere.io/dev/HTTP_API_Projects_Products.html#product-projections-by-query]].
    * @return <a href="HTTP_API.html#paged-query-response">PagedQueryResult</a>
    *         with the <code>results</code> array of <a href="#product-projection">ProductProjection</a> JSON. */
  def getProducts(oauthToken: String): Future[JValue] = {
    h(url(s"${ApiUrl}${projectKey}/product-projections").setHeader("Authorization", s"Bearer ${oauthToken}"))
      .map(rsp => parse(rsp.getResponseBody))
  }

  val f = for {
    token <- login(clientId, clientSecret, projectKey)
    products <- getProducts(token)
  } yield products \ "total" match {
    case JInt(n) => n
    case _ => throw new IllegalStateException("Product query response does not contain 'total'.")
  }

  val nrOfProducts = Await.result(f, Duration.Inf)
  h.shutdown()
  println("Number of products: " + nrOfProducts)
}