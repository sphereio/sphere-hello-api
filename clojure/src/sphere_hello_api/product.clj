(ns sphere-hello-api.product 
  (:require [clj-http.client :as client]
            [clojure.data.codec.base64 :as base64]
            [clojure.data.json :as json]))

(def config 
  {:api-url "https://api.sphere.io"
   :auth-api-url "https://auth.sphere.io/oauth/token"

   ;; TODO: fill in your sphere credentials
   :client-id ""
   :client-secret ""
   :project-key ""})

(defn encode [s]
  (String. (base64/encode (.getBytes s)) "UTF-8"))
    
(defn login 
  "Retrieves oauth access token"
  [{:keys [auth-api-url client-id client-secret project-key]}]
  (let [auth-token (str "Basic " (encode (str client-id ":" client-secret)))
        auth-response (client/post auth-api-url
                        {:headers {"Authorization" auth-token
                                   "Content-Type" "application/x-www-form-urlencoded"}
                         :body (str "grant_type=client_credentials&scope=manage_project:" project-key)})]
    ((json/read-str (:body auth-response)) "access_token")))

(defn -main [& args]
  (let [access-token (login config)
        products-response (client/get 
                            (str (:api-url config) "/" (:project-key config) "/product-projections")
                            {:headers {"Authorization" (str "Bearer " access-token)}})]
    (println "Number of products:" ((json/read-str (:body products-response)) "total"))))