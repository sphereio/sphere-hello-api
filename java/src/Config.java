import io.sphere.client.shop.ShopClientConfig;
import java.io.FileInputStream;
import java.io.IOException;
import java.util.Properties;

public final class Config {
    /** Creates a {@link ShopClientConfig} based on a properties file. */
    public static ShopClientConfig load(String filename) throws IOException {
        Properties p = new Properties();
        p.load(new FileInputStream(filename));
        checkValid(p);
        return new ShopClientConfig.Builder(
                p.getProperty("projectKey"), p.getProperty("clientId"), p.getProperty("clientSecret")).build();
    }

    private static void checkValid(Properties p) {
        if (p.getProperty("projectKey").equals("my-project-key"))
            throw new RuntimeException("Please provide your project credentials first. See the 'config.properties' file.");
    }
}
