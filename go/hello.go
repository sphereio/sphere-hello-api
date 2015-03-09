package main

import (
  "bytes"
  "encoding/json"
  "fmt"
  "io/ioutil"
  "log"
  "net/http"
  "net/url"
  "strconv"
  "github.com/sphereio/jsonq"
)

type Credentials struct {
  ClientId string `json:"client_id"`
  ClientSecret string `json:"client_secret"`
  ProjectKey string `json:"project_key"`
}

type Auth struct {
  AccessToken string `json:"access_token"`
}

func parseJson(path string) *Credentials {
  file, e := ioutil.ReadFile(path)
  if e != nil {
    log.Fatal("Error while reading config file: ", e)
  }
  var c Credentials
  json.Unmarshal(file, &c)
  return &c
}

func getAccessToken(c *Credentials) *Auth {
  authUrl := fmt.Sprintf("https://%v:%v@auth.sphere.io/oauth/token", c.ClientId, c.ClientSecret)
  data := url.Values{}
  data.Set("grant_type", "client_credentials")
  data.Set("scope", fmt.Sprintf("manage_project:%v", c.ProjectKey))

  client := &http.Client{}
  req, _ := http.NewRequest("POST", authUrl, bytes.NewBufferString(data.Encode()))
  req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
  req.Header.Add("Content-Length", strconv.Itoa(len(data.Encode())))

  resp, err := client.Do(req)
  if err != nil {
    log.Fatal("Error when retrieving access_token: ", err)
  }
  defer resp.Body.Close()
  var a Auth
  json.NewDecoder(resp.Body).Decode(&a)
  return &a
}

func getProductProjections(access_token string, project string) *jsonq.JsonQuery {
  apiUrl := fmt.Sprintf("https://api.sphere.io/%v/product-projections", project)

  client := &http.Client{}
  req, _ := http.NewRequest("GET", apiUrl, nil)
  req.Header.Add("Content-Type", "application/json")
  req.Header.Add("Authorization", fmt.Sprintf("Bearer %v", access_token))

  resp, err := client.Do(req)
  if err != nil {
    log.Fatal("Error when fetching products: ", err)
  }
  defer resp.Body.Close()
  // we now use a json parsing library to decode arbitrary data
  // in order to avoid declaring structs for the entire products response
  data := map[string]interface{}{}
  json.NewDecoder(resp.Body).Decode(&data)
  jq := jsonq.NewQuery(data)

  return jq
}

func main() {
  // read credentials
  c := parseJson("./config.json")

  // get an access_token
  auth := getAccessToken(c)

  // get product projections
  products := getProductProjections(auth.AccessToken, c.ProjectKey)
  count, _ := products.Int("count")
  fmt.Println(fmt.Sprintf("Found %v products", count))
}
