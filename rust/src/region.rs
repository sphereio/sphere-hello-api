use std::str::FromStr;

#[derive(PartialEq)]
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
            _ => Err(format!("Region '{}' not recognized. Valid values: 'Europe', 'NorthAmerica'.", s)),
        }

    }
}

impl Region {
    pub fn api_url(&self) -> String {
        match self {
            &Region::Europe => "https://api.sphere.io".to_string(),
            &Region::NorthAmerica => "https://api.commercetools.co".to_string(),
        }
    }

    pub fn auth_url(&self) -> String {
        match self {
            &Region::Europe => "https://auth.sphere.io".to_string(),
            &Region::NorthAmerica => "https://auth.commercetools.co".to_string(),
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
        assert!(Region::from_str("India") == Result::Err("Region 'India' not recognized. Valid values: 'Europe', 'NorthAmerica'.".to_string()));
    }
}
