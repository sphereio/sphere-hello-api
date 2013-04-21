module Main where

import Control.Monad (forM_)
import Network.Curl (withCurlDo)
import Network.Curl.Easy (initialize)
import System.Environment (getArgs)
import System.Exit (exitFailure)
import System.IO (hPutStrLn, stderr)

import SphereHelloApi.ApiAccess
import qualified SphereHelloApi.Products as Products
import qualified SphereHelloApi.Tokens as Tokens

main :: IO ()
main = withCurlDo $ do
   c <- initialize
   cfg <- getArgs >>= (\args ->
            case args of
               [file] -> loadConfig file
               _ -> (hPutStrLn stderr $ "Usage: sphere-hello-api <config>") >> exitFailure)
   t <- Tokens.getToken c cfg
   -- putStrLn $ "Your access token: " ++ Tokens.access_token t
   p <- Products.getProducts c cfg t
   putStrLn $ "You have " ++ show (Products.count p) ++ " product(s):"
   forM_ (Products.results p) (putStrLn . show)
