import io.sphere.client.shop.ShopClient;
import io.sphere.client.shop.model.Product;

import java.io.IOException;

// This example uses the official Sphere Java client:
// https://github.com/commercetools/sphere-play-sdk

public class HelloApi {
    public static void main(String[] args) throws IOException {
        ShopClient sphere = ShopClient.create(Config.load("config.properties"));

        for (Product product : sphere.products().all().fetch().getResults()) {
            System.out.println(product.getName());
        }
    }
}
