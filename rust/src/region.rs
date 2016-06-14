use std::str::FromStr;

#[derive(PartialEq)]
/// World region for which the commercetools platform has data-centers
pub enum Region {
    Europe,
    NorthAmerica,
}

impl FromStr for Region {
    type Err = String;
    fn from_str(s: &str) -> Result<Region, String> {
        match s.to_string().to_lowercase().as_ref() {
            "europe" => Ok(Region::Europe),
            "northamerica" => Ok(Region::NorthAmerica),
            _ => {
                Err(format!("Region '{}' not recognized. Valid values: 'Europe', 'NorthAmerica'.",
                            s))
            }
        }
    }
}

impl Region {
    /// Returns the [api url](http://dev.commercetools.com/http-api.html#hosts) for this region
    pub fn api_url(&self) -> &str {
        match self {
            &Region::Europe => "https://api.sphere.io",
            &Region::NorthAmerica => "https://api.commercetools.co",
        }
    }

    /// Returns the [auth url](http://dev.commercetools.com/http-api-authorization.html#hosts) for this region
    pub fn auth_url(&self) -> &str {
        match self {
            &Region::Europe => "https://auth.sphere.io",
            &Region::NorthAmerica => "https://auth.commercetools.co",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn region_from_string() {
        assert!(Region::from_str("Europe").unwrap() == Region::Europe);
        assert!(Region::from_str("europe").unwrap() == Region::Europe);
        assert!(Region::from_str("NorthAmerica").unwrap() == Region::NorthAmerica);
        assert!(Region::from_str("northAmeriCA").unwrap() == Region::NorthAmerica);
    }

    #[test]
    fn unparsable_region_err() {
        assert!(Region::from_str("India") ==
                Result::Err("Region 'India' not recognized. Valid values: 'Europe', 'NorthAmerica'.".to_string()));
    }
}
