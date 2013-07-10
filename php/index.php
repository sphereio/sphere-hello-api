<?php
  include('./config.php');
  include('./app.php');

  $encoded_products = json_encode($result); // to json string
  $products = json_decode($encoded_products); // to object

  function imgUrl($url, $size){
    $parts = explode(".", $url);
    $idx = count($parts) - 2;
    $name = $parts[$idx]."-".$size;
    $parts[$idx] = $name;
    return implode(".", $parts);
  }
  foreach($products->results as $product) {
?>
<div style="height: 190px;
  width: 155px;
  margin: 10px 25px 0 0;
  float: left;
  font-size: 11px;
  color: #6d778e;
  font-family: 'Open Sans',sans-serif;
  font-weight: 400;
  ">
  <a href="#" style="color: #00b5de;">
    <img src="<?= count($product->masterVariant->images) > 0 ? imgUrl($product->masterVariant->images[0]->url, "small") : "http://placehold.it/165.png" ?>" style="width: 140px;
      height: 140px;
      display: block;
      padding: 4px;
      max-width: 100%;
      vertical-align: middle;
      background-color: #fff;
      border: 1px solid rgba(0,0,0,0.2);
      -webkit-box-shadow: 0 1px 3px rgba(0,0,0,0.1);
      -moz-box-shadow: 0 1px 3px rgba(0,0,0,0.1);
      box-shadow: 0 1px 3px rgba(0,0,0,0.1);
      ">
    <span style="text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      display: block;
      "><?= $product->name->en ?></span>
  </a>
  <span style="text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
    display: block;
    "><?= count($product->variants) ?> Variants</span>
</div>
<?php } ?>