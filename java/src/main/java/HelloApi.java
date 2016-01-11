import io.sphere.sdk.client.BlockingSphereClient;
import io.sphere.sdk.client.SphereClient;
import io.sphere.sdk.client.SphereClientConfig;
import io.sphere.sdk.client.SphereClientFactory;
import io.sphere.sdk.products.ProductProjection;
import io.sphere.sdk.products.search.ProductProjectionSearch;

import java.io.IOException;
import java.io.InputStream;
import java.util.List;
import java.util.Properties;
import java.util.concurrent.TimeUnit;

// This example uses the official Sphere Java client:
// https://github.com/sphereio/sphere-jvm-sdk

public class HelloApi {
    public static void main(String[] args) throws Exception {
        try(final BlockingSphereClient client = createCommercetoolsClient()) {
            final ProductProjectionSearch sphereRequest = ProductProjectionSearch.ofCurrent()
                    .withSort(product -> product.createdAt().byDesc());
            final List<ProductProjection> products = client.executeBlocking(sphereRequest).getResults();
            for (final ProductProjection product : products) {
                System.out.println(product.getName());
            }
        }
    }

    private static BlockingSphereClient createCommercetoolsClient() throws IOException {
        final Properties properties = loadPropertiesFromClasspath("commercetools.properties");
        final SphereClientConfig config = SphereClientConfig.ofProperties(properties, "");
        final SphereClient underlyingClient = SphereClientFactory.of().createClient(config);//this client only works asynchronous
        return BlockingSphereClient.of(underlyingClient, 10, TimeUnit.SECONDS);
    }

    private static Properties loadPropertiesFromClasspath(final String path) throws IOException {
        final Properties properties = new Properties();
        try (final InputStream stream = HelloApi.class.getResourceAsStream(path)) {
            properties.load(stream);
        }
        return properties;
    }
}
