name := "sphere-hello-api"

description := "An example application that authenticates and access project data using SPHERE.IO API"

scalaVersion := "2.11.4"

libraryDependencies ++=
  "net.databinder.dispatch" %% "dispatch-core" % "0.11.2" ::
  "net.liftweb" %% "lift-json" % "2.6-RC2" ::
  "com.typesafe" % "config" % "1.2.1" ::
  "ch.qos.logback" % "logback-classic" % "1.1.2" ::
  Nil