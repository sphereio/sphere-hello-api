import sbt._
import Keys._

object HelloBuild extends Build {

    lazy val root = Project(
      id = "sphere-hello",
      base = file("."),
      settings = Project.defaultSettings ++ Seq(
        scalaVersion := "2.10.0",
        libraryDependencies ++= Seq(
          "net.databinder.dispatch" %% "dispatch-core" % "0.10.0",
          "net.liftweb" %% "lift-json" % "2.5-RC5",
          "com.typesafe" % "config" % "1.0.0")))
}
