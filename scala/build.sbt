name := "sphere-hello-api"

description := "An example application that authenticates and access project data using SPHERE.IO API"

scalaVersion := "2.10.3"

libraryDependencies ++= Seq(
  "net.databinder.dispatch" % "dispatch-core_2.10" % "0.10.0",
  "net.liftweb" % "lift-json_2.10" % "2.5-RC5",
  "com.typesafe" % "config" % "1.0.0"
)
