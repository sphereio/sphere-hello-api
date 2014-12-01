
# A minimal SPHERE.IO backed webpage in Groovy

This is just a Groovy _script_ to play around. For real Groovy-based solutions please do all SPHERE.IO API access via the JVM SDK (https://github.com/sphereio/sphere-jvm-sdk)

## Prerequisite
* Groovy 1.8.6 (or higher) and a Java 7 JVM (or higher). Working "groovy" and "grape" commands in the path.
* Internet Access on first start to download the Jetty jar

## How to run
 * create an account and project at http://admin.sphere.io if you don't have one yet
 * navigate to developers -> API clients and copy your permissions and project name in the config.groovy file.
 * run `groovy ./server.groovy`
 * open http://localhost:8080/index.groovy in your Browser
 * start playing around in the template part of index.groovy, API documentation can be found here: http://dev.sphere.io/ . See also: http://groovy.codehaus.org/Groovlets with the MarkupBuilder



