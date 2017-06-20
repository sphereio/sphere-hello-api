use std::str::FromStr;

pub trait HasAuthUrl {
    fn auth_url(&self) -> String;
}

pub trait HasApiUrl {
    fn api_url(&self) -> String;
}

#[derive(PartialEq, Eq)]
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

impl HasAuthUrl for Region {
    /// Returns the [auth url](http://dev.commercetools.com/http-api-authorization.html#hosts)
    /// for this region
    fn auth_url(&self) -> String {
        match *self {
            Region::Europe => String::from("https://auth.sphere.io"),
            Region::NorthAmerica => String::from("https://auth.commercetools.co"),
        }
    }
}

impl HasApiUrl for Region {
    /// Returns the [api url](http://dev.commercetools.com/http-api.html#hosts) for this region
    fn api_url(&self) -> String {
        match *self {
            Region::Europe => String::from("https://api.sphere.io"),
            Region::NorthAmerica => String::from("https://api.commercetools.co"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn region_from_string() {
        assert!(Region::from_str("Europe") == Ok(Region::Europe));
        assert!(Region::from_str("europe") == Ok(Region::Europe));
        assert!(Region::from_str("NorthAmerica") == Ok(Region::NorthAmerica));
        assert!(Region::from_str("northAmeriCA") == Ok(Region::NorthAmerica));
    }

    #[test]
    fn unparsable_region_err() {
        assert!(Region::from_str("India") ==
                Result::Err("Region 'India' not recognized. Valid values: 'Europe', \
                             'NorthAmerica'."
                                    .to_string()));
    }
}
