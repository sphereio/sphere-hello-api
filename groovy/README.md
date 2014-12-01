
# A minimal SPHERE.IO API access in groovy

This are just two Groovy _scripts_ to play around. For real Groovy-based solutions please do all SPHERE.IO API access via the JVM SDK (https://github.com/sphereio/sphere-jvm-sdk). For production you may also want to take a look at High Performance Groovy-Capable Microframeworks like http://ratpack.io


## Prerequisite
* Groovy 1.8.6 (or higher) and a Java 7 JVM (or higher). Working "groovy" (and optinally "grape") command in the path.
* Internet Access 

## How to run
 1. Create an account and project at http://admin.sphere.io if you don't have one yet
 2. Navigate to developers -> API clients and copy your permissions and project key into the config.groovy file.
 3. Get API data:
   * just call it via `groovy apidump.groovy`
   * play with adding calls to other endpoints and play with the results. The SPHERE.IO API documentation at http://dev.sphere.io/ is your friend
 4. (optional) If you feel like wanting show a Browser window, start the minimal webserver (depending on your environment you may have issues with the automatic jetty and servlet API @Grab downloads)
   * run `groovy ./server.groovy`
   * open http://localhost:8080/index.groovy in your Browser
   * start playing around in the template part of index.groovy. See http://groovy.codehaus.org/Groovlets with the MarkupBuilder

