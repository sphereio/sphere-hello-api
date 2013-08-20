//
//  CTMasterViewController.h
//  HelloAPI
//
//  Created by Leonard Ehrenfried on 8/20/13.
//  Copyright (c) 2013 Commercetools. All rights reserved.
//

#import <UIKit/UIKit.h>

@class CTDetailViewController;

@interface CTMasterViewController : UITableViewController

@property (strong, nonatomic) CTDetailViewController *detailViewController;

@end
