{-# LANGUAGE DeriveDataTypeable #-}
module SphereHelloApi.Products (getProducts, ProductResponse (..), Product (..)) where

import Network.Curl
import SphereHelloApi.ApiAccess
import SphereHelloApi.Tokens
import Text.JSON.Generic

getProductsOpts t =
   -- CurlVerbose True :
   CurlHttpHeaders ["Authorization: Bearer " ++ access_token t] :
   method_GET

getProducts :: Curl -> ApiConfig -> TokenResponse -> IO ProductResponse
getProducts c cfg t = do
   resp <- do_curl_ c (productsUrl cfg) (getProductsOpts t) :: IO CurlResponse
   -- putStrLn $ respBody resp
   return $ decodeJSON (respBody resp)

data ProductResponse = ProductResponse {
     total :: Int
   , count :: Int
   , offset :: Int
   , results :: [Product]
} deriving (Eq, Show, Data, Typeable)

-- For all the available data see: http://sphere.io/dev/HTTP_API_Projects_Products.html#product-projection
data Product = Product {
     name :: String
   , description :: String
   , slug :: String
} deriving (Eq, Show, Data, Typeable)