import io.sphere.client.shop.SphereClient;
import io.sphere.client.shop.model.Product;

import java.io.IOException;

// This example uses the official Sphere Java client:
// https://github.com/commercetools/sphere-play-sdk

public class HelloApi {
    public static void main(String[] args) throws IOException {
        SphereClient sphere = SphereClient.create(Config.load("src/main/resources/commercetools.properties"));

        for (Product product : sphere.products().all().fetch().getResults()) {
            System.out.println(product.getName());
        }
        sphere.shutdown();
    }
}
