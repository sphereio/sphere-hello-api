{-# LANGUAGE DeriveDataTypeable #-}
module SphereHelloApi.Tokens (TokenResponse (..), getToken) where

import Network.Curl
import Text.JSON.Generic
import SphereHelloApi.ApiAccess

getTokenOpts :: ApiConfig -> [CurlOption]
getTokenOpts cfg =
   -- CurlVerbose True :
   CurlPostFields ["grant_type=client_credentials&scope=manage_project:" ++ projectKey cfg] :
   CurlUserPwd (clientId cfg ++ ":" ++ clientSecret cfg) :
   method_POST

getToken :: Curl -> ApiConfig -> IO TokenResponse
getToken c cfg = do
   resp <- do_curl_ c tokenUrl (getTokenOpts cfg) :: IO CurlResponse
   return $ decodeJSON (respBody resp)

data TokenResponse = TokenResponse {
     access_token :: String
   , token_type :: String
   , expires_in :: Int
   , scope :: String
} deriving (Eq, Show, Data, Typeable)
