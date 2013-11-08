import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.{Future, Await}
import scala.concurrent.duration.Duration
import com.typesafe.config._
import com.ning.http.client.{Response, AsyncHttpClient}
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

  def prettyResponeBody(resp: Response) =
    parseOpt(resp.getResponseBody) map (json => pretty(render(json))) getOrElse resp.getResponseBody

  /** @return oauth access token */
  def login(clientId: String, clientSecret: String, projectKey: String): Future[String] = {
    val encoded = Base64.encode(s"$clientId:$clientSecret".getBytes)
    h(url(AuthApiUrl).POST
      .setHeader("Authorization", s"Basic $encoded")
      .setHeader("Content-Type", "application/x-www-form-urlencoded")
      .setBody(s"grant_type=client_credentials&scope=manage_project:$projectKey")
    ).map(rsp => (rsp.getStatusCode, parseOpt(rsp.getResponseBody) map (_ \ "access_token")) match {
      case (200, Some(JString(s))) => s
      case (status, _) =>
        throw new IllegalStateException("Auth response does not contain 'access_token'. " +
          s"HTTP status: $status. ${prettyResponeBody(rsp)}")
    })
  }
  
  /** Issues a query to get all products, see [[http://sphere.io/dev/HTTP_API_Projects_Products.html#product-projections-by-query]].
    * @return <a href="HTTP_API.html#paged-query-response">PagedQueryResult</a>
    *         with the <code>results</code> array of <a href="#product-projection">ProductProjection</a> JSON. */
  def getProducts(oauthToken: String): Future[Response] =
    h(url(s"${ApiUrl}$projectKey/product-projections").setHeader("Authorization", s"Bearer $oauthToken"))

  try {
    val f = for {
      token <- login(clientId, clientSecret, projectKey)
      productResp <- getProducts(token)
    } yield (productResp.getStatusCode, parseOpt(productResp.getResponseBody) map (_ \ "total")) match {
        case (200, Some(JInt(total))) => total
        case (status, _) =>
          throw new IllegalStateException("Product query response is invalid ot does not contain 'total'. " +
            s"HTTP status $status. ${prettyResponeBody(productResp)}")
      }

    val nrOfProducts = Await.result(f, Duration.Inf)

    println("Number of products: " + nrOfProducts)
  } finally {
    h.shutdown()
  }
}