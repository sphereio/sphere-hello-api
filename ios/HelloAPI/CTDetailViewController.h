//
//  CTDetailViewController.h
//  HelloAPI
//
//  Created by Leonard Ehrenfried on 8/20/13.
//  Copyright (c) 2013 Commercetools. All rights reserved.
//

#import <UIKit/UIKit.h>

@interface CTDetailViewController : UIViewController <UISplitViewControllerDelegate>

@property (strong, nonatomic) id detailItem;

@property (weak, nonatomic) IBOutlet UILabel *detailDescriptionLabel;
@end
