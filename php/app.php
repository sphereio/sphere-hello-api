<?php
  include('./config.php');

  function makeRequest($url, $context) {
    $fp = fopen($url, 'rb', false, $context);
    if (!$fp) {
      throw new Exception("Problem with $url");
    }
    // get the response and decode
    $response = stream_get_contents($fp);
    if ($response === false) {
      throw new Exception("Problem reading data from $url");
    }
    $result = json_decode($response, true);
    // close the response
    fclose($fp);

    return $result;
  }

  // Request AccessToken
  $authUrl = "https://$client_id:$client_secret@auth.sphere.io/oauth/token";
  $data = array("grant_type" => "client_credentials", "scope" => "manage_project:$project_key");
  $options = array(
    'http' => array(
      'header'  => "Content-type: application/x-www-form-urlencoded\r\n",
      'method'  => 'POST',
      'content' => http_build_query($data),
    ),
  );
  $context = stream_context_create($options);
  $authResult = makeRequest($authUrl, $context);
  $access_token = $authResult["access_token"];

  // Fetch products
  $productUrl = "https://api.sphere.io/$project_key/product-projections";
  $options = array(
    'http' => array(
      'header'  => "Authorization: Bearer $access_token",
      'method'  => 'GET'
    ),
  );
  $c = stream_context_create($options);
  $result = makeRequest($productUrl, $c); // array

  // Print result as JSON
  //header('Content-Type: application/json');
  //echo json_encode($result)
?>