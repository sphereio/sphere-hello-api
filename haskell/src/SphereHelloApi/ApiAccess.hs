{-# LANGUAGE DeriveDataTypeable #-}
module SphereHelloApi.ApiAccess where

import Control.Monad (liftM)
import Text.JSON.Generic

data ApiConfig = ApiConfig {
      projectKey :: String
    , clientId :: String
    , clientSecret :: String
} deriving (Eq, Show, Data, Typeable)

loadConfig :: String -> IO ApiConfig
loadConfig = (liftM decodeJSON) . readFile

productsUrl :: ApiConfig -> String
productsUrl cfg = "https://api-v0.sphere.io/" ++ projectKey cfg ++ "/product-projections"

tokenUrl = "https://auth-v0.sphere.io/oauth/token"
