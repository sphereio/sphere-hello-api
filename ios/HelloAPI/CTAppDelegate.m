//
//  CTAppDelegate.m
//  HelloAPI
//
//  Created by Leonard Ehrenfried on 8/20/13.
//  Copyright (c) 2013 Commercetools. All rights reserved.
//

#import "CTAppDelegate.h"

#import "CTMasterViewController.h"

#import "CTDetailViewController.h"

#import <Base64/MF_Base64Additions.h>
#import <AFNetworking/AFNetworking.h>


@implementation CTAppDelegate

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions
{
    self.window = [[UIWindow alloc] initWithFrame:[[UIScreen mainScreen] bounds]];
    
    // Override point for customization after application launch.
    if ([[UIDevice currentDevice] userInterfaceIdiom] == UIUserInterfaceIdiomPhone) {
        CTMasterViewController *masterViewController = [[CTMasterViewController alloc] initWithNibName:@"CTMasterViewController_iPhone" bundle:nil];
        self.masterViewController = masterViewController;
        self.navigationController = [[UINavigationController alloc] initWithRootViewController:masterViewController];
        self.window.rootViewController = self.navigationController;
    } else {
        CTMasterViewController *masterViewController = [[CTMasterViewController alloc] initWithNibName:@"CTMasterViewController_iPad" bundle:nil];
        self.masterViewController = masterViewController;
        UINavigationController *masterNavigationController = [[UINavigationController alloc] initWithRootViewController:masterViewController];
        
        CTDetailViewController *detailViewController = [[CTDetailViewController alloc] initWithNibName:@"CTDetailViewController_iPad" bundle:nil];
        UINavigationController *detailNavigationController = [[UINavigationController alloc] initWithRootViewController:detailViewController];
        
        masterViewController.detailViewController = detailViewController;
        
        self.splitViewController = [[UISplitViewController alloc] init];
        self.splitViewController.delegate = detailViewController;
        self.splitViewController.viewControllers = @[masterNavigationController, detailNavigationController];
        
        self.window.rootViewController = self.splitViewController;
    }

    NSString *path = [[NSBundle mainBundle] pathForResource:@"project" ofType:@"plist"];
    NSDictionary *settings = [[NSDictionary alloc] initWithContentsOfFile:path];
    NSString *projectKey    = [settings objectForKey:@"projectKey"];
    NSString *clientId      = [settings objectForKey:@"clientId"];
    NSString *clientSecret  = [settings objectForKey:@"clientSecret"];

    [self fetchAuthorization:projectKey clientId:clientId clientSecret:clientSecret];
    [self.window makeKeyAndVisible];
    return YES;
}

- (void)fetchAuthorization:(NSString *)projectKey clientId:(NSString *)clientId clientSecret:(NSString *)clientSecret
{
    NSMutableURLRequest *request = [NSMutableURLRequest requestWithURL:[NSURL URLWithString:@"https://auth.sphere.io/oauth/token"]];
    request.HTTPMethod = @"POST";
    [request setValue:@"application/x-www-form-urlencoded" forHTTPHeaderField:@"Content-Type"];
    
    NSString *auth      = [NSString stringWithFormat:@"%@:%@", clientId, clientSecret];
    NSString *header    = [NSString stringWithFormat:@"Basic %@", [auth base64String]];
    [request setValue:header forHTTPHeaderField:@"Authorization"];
    
    NSString *body = [NSString stringWithFormat:@"grant_type=client_credentials&scope=manage_project:%@", projectKey];
    request.HTTPBody = [body dataUsingEncoding:NSUTF8StringEncoding];
    
    AFJSONRequestOperation *operation =
    [AFJSONRequestOperation JSONRequestOperationWithRequest:request
                                                    success:^(NSURLRequest *request, NSHTTPURLResponse *response, id json) {
                                                        NSDictionary *dict = (NSDictionary*) json;
                                                        [self fetchProducts:projectKey token:[dict objectForKey:@"access_token"]];
                                                    }
                                                    failure:^(NSURLRequest *request, NSHTTPURLResponse *response, NSError *error, id JSON) {
                                                        UIAlertView *av = [[UIAlertView alloc] initWithTitle:@"Authenticateion error"
                                                                                                     message:[NSString stringWithFormat:@"%@",error]
                                                                                                    delegate:nil
                                                                                           cancelButtonTitle:@"OK" otherButtonTitles:nil];
                                                        [av show];
                                                    }];
    
    [operation start];
}

- (void)fetchProducts:(NSString *)projectKey token:(NSString *)token
{
    NSString * url = [NSString stringWithFormat: @"https://api.sphere.io/%@/product-projections", projectKey];
    NSMutableURLRequest *request = [NSMutableURLRequest requestWithURL:[NSURL URLWithString:url]];
    
    NSString *header = [NSString stringWithFormat:@"Bearer %@", token];
    [request setValue:header forHTTPHeaderField:@"Authorization"];
    
    AFJSONRequestOperation *operation =
    [AFJSONRequestOperation JSONRequestOperationWithRequest:request
                                                    success:^(NSURLRequest *request, NSHTTPURLResponse *response, id json) {
                                                        NSDictionary *dict = (NSDictionary*) json;
                                                        NSArray *products = [dict objectForKey:@"results"];
                                                        self.masterViewController.products = products;
                                                        [self.masterViewController.tableView reloadData];                                                        
                                                    }
                                                    failure:^(NSURLRequest *request, NSHTTPURLResponse *response, NSError *error, id JSON) {
                                                        UIAlertView *av = [[UIAlertView alloc] initWithTitle:@"Error fetching products"
                                                                                                     message:[NSString stringWithFormat:@"%@",error]
                                                                                                    delegate:nil
                                                                                           cancelButtonTitle:@"OK" otherButtonTitles:nil];
                                                        [av show];
                                                    }];
    
    [operation start];
}


- (void)applicationWillResignActive:(UIApplication *)application
{
    // Sent when the application is about to move from active to inactive state. This can occur for certain types of temporary interruptions (such as an incoming phone call or SMS message) or when the user quits the application and it begins the transition to the background state.
    // Use this method to pause ongoing tasks, disable timers, and throttle down OpenGL ES frame rates. Games should use this method to pause the game.
}

- (void)applicationDidEnterBackground:(UIApplication *)application
{
    // Use this method to release shared resources, save user data, invalidate timers, and store enough application state information to restore your application to its current state in case it is terminated later. 
    // If your application supports background execution, this method is called instead of applicationWillTerminate: when the user quits.
}

- (void)applicationWillEnterForeground:(UIApplication *)application
{
    // Called as part of the transition from the background to the inactive state; here you can undo many of the changes made on entering the background.
}

- (void)applicationDidBecomeActive:(UIApplication *)application
{
    // Restart any tasks that were paused (or not yet started) while the application was inactive. If the application was previously in the background, optionally refresh the user interface.
}

- (void)applicationWillTerminate:(UIApplication *)application
{
    // Called when the application is about to terminate. Save data if appropriate. See also applicationDidEnterBackground:.
}

@end
