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
        final SphereClientConfig config = loadCommercetoolsConfig();
        final SphereClientFactory factory = SphereClientFactory.of();
        final SphereClient underlyingClient = factory.createClient(config);//this client only works asynchronous
        try(final BlockingSphereClient client = BlockingSphereClient.of(underlyingClient, 10, TimeUnit.SECONDS)) {
            final ProductProjectionSearch sphereRequest = ProductProjectionSearch.ofCurrent()
                    .withSort(m -> m.createdAt().byDesc());
            final List<ProductProjection> products = client.executeBlocking(sphereRequest).getResults();
            for (ProductProjection product : products) {
                System.out.println(product.getName());
            }
        }
    }

    private static SphereClientConfig loadCommercetoolsConfig() throws IOException {
        final Properties properties = loadPropertiesFromClasspath("commercetools.properties");
        return SphereClientConfig.ofProperties(properties, "");
    }

    private static Properties loadPropertiesFromClasspath(final String path) throws IOException {
        final Properties properties = new Properties();
        try (final InputStream stream = HelloApi.class.getResourceAsStream(path)) {
            properties.load(stream);
        }
        return properties;
    }
}
