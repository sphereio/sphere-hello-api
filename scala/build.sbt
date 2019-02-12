name := "sphere-hello-api"

description := "An example application that authenticates and access project data using SPHERE.IO API"

scalaVersion := "2.12.8"

libraryDependencies ++=
  "org.dispatchhttp" %% "dispatch-core" % "0.14.0" ::
  "net.liftweb" %% "lift-json" % "3.1.1" ::
  "com.typesafe" % "config" % "1.3.3" ::
  "ch.qos.logback" % "logback-classic" % "1.2.3" ::
  Nil
