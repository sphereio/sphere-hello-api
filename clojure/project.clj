(defproject sphere-hello-api "1.0"
  :description "An example application that authenticates and access project data using SPHERE.IO API"
  :main sphere-hello-api.product
  :dependencies 
  	[[org.clojure/clojure "1.10.1"] 
  	 [clj-http "0.9.2"]
  	 [org.clojure/data.codec "0.1.1"]
  	 [org.clojure/data.json "0.2.7"]
  	 [sonian/carica "1.2.2"]])
