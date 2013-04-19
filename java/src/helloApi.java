import io.sphere.client.SphereException;
import io.sphere.client.shop.ShopClient;
import io.sphere.client.shop.ShopClientConfig;
import io.sphere.client.shop.ApiMode;
import io.sphere.client.shop.model.Product;
import io.sphere.client.model.SearchResult;

import java.util.List;
import org.apache.commons.configuration.Configuration;
import org.apache.commons.configuration.PropertiesConfiguration;
import org.apache.commons.configuration.ConfigurationException;
import org.apache.commons.configuration.ConversionException;

public class HelloApi {
    public static void main(String[] args) {
        try {
            Configuration config = readConfiguration();       
        	
            ShopClient client = buildClient(
                config.getString("projectKey"),
                config.getString("clientId"),
                config.getString("clientSecret"));

        	listProducts(client);

        } catch (ConfigurationException ce) {
            System.err.println("Could not open configuration file 'config.properties'");
        } catch (ConversionException ce) {
            System.err.println("Could not read configuration values");
        } catch (SphereException se) {
            se.printStackTrace();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static Configuration readConfiguration() throws ConfigurationException {
        return new PropertiesConfiguration("config.properties");
    }

    public static ShopClient buildClient(String project, String client, String secret) {
        ShopClientConfig config = new ShopClientConfig
                .Builder(project, client, secret)
                .setApiMode(ApiMode.Live)
                .build();
        return ShopClient.create(config);
    }

    public static void listProducts(ShopClient client) {
        SearchResult<Product> result = client.products().all().pageSize(1000).fetch();
        for (Product product : result.getResults()) {
            System.out.println(product.getName());
        }
    }
}
