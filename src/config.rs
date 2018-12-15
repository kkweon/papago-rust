#[derive(Debug, Serialize, Deserialize)]
pub struct NaverAuth {
    pub client_id: String,
    pub client_secret: String,
}

impl NaverAuth {
    pub fn new<'a>(client_id: &'a str, client_secret: &'a str) -> NaverAuth {
        NaverAuth {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
        }
    }
}

impl PartialEq for NaverAuth {
    fn eq(&self, other: &NaverAuth) -> bool {
        self.client_id == other.client_id && self.client_secret == other.client_secret
    }
}

#[derive(Debug)]
pub enum NaverAuthError {
    NoClientId,
    NoClientSecret,
}

pub fn get_auth_from_env() -> Result<NaverAuth, NaverAuthError> {
    let client_id = std::env::var("NAVER_CLIENT_ID");
    if client_id.is_err() {
        return Err(NaverAuthError::NoClientId);
    }

    let client_secret = std::env::var("NAVER_CLIENT_SECRET");
    if client_secret.is_err() {
        return Err(NaverAuthError::NoClientSecret);
    }

    Ok(NaverAuth {
        client_id: client_id.unwrap(),
        client_secret: client_secret.unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let auth = NaverAuth::new("client_id", "client_secret");
        assert_eq!(
            auth,
            NaverAuth {
                client_id: "client_id".to_owned(),
                client_secret: "client_secret".to_owned(),
            }
        );
    }

    #[test]
    fn test_get_env() {
        let auth = get_auth_from_env();
        assert!(auth.is_ok());
    }
}
