pub use self::model::Lang;

pub use self::model::PapagoReq;

use crate::config::NaverAuth;

mod model;

/// NMT_URL uses Neural Net Machine Translation
pub const NMT_URL: &'static str = "https://openapi.naver.com/v1/papago/n2mt";
/// SMT_URL uses Statistical Model
pub const SMT_URL: &'static str = "https://openapi.naver.com/v1/language/translate";

/// Error Type
#[derive(PartialEq)]
pub enum Error {
    WrongPayload,
    ReqFail,
    RespParseFail,
    PapagoError(String),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::WrongPayload => write!(f, "Request payload is not correct"),
            Error::ReqFail => write!(
                f,
                "Request has failed. Maybe check your internet connection."
            ),
            Error::RespParseFail => write!(
                f,
                "Response was not something expected. Please report if it's a bug"
            ),
            Error::PapagoError(msg) => write!(f, "{}", msg),
        }
    }
}

/// Returns Translation Result
///
/// # Arguments
///
/// * `url` - Use `SMT_URL` or `NMT_URL`
/// * `auth` - Naver ClientId and ClientSecret
/// * `payload` - Payload
///
/// # Example
///
/// ```rust
/// # use papago::api::*;
/// # use papago::config;
/// let auth = config::get_auth_from_env().unwrap();
/// let payload = PapagoReq {
///     source: Lang::En,
///     target: Lang::Ko,
///     text: "HELLO WORLD".to_owned(),
/// };
/// let resp = send_request(NMT_URL, &auth, payload).unwrap();
/// assert_eq!(resp, "안녕 세계".to_owned());
/// ```
pub fn send_request(
    url: &'static str,
    auth: &NaverAuth,
    payload: model::PapagoReq,
) -> Result<String, Error> {
    let body = serde_urlencoded::to_string(&payload).or_else(|_| Err(Error::WrongPayload))?;

    let client = reqwest::Client::new();
    let mut resp = client
        .post(url)
        .header("X-Naver-Client-Id", auth.client_id.clone())
        .header("X-Naver-Client-Secret", auth.client_secret.clone())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .or(Err(Error::ReqFail))?;

    let response: model::PapagoResponse = resp.json().or(Err(Error::RespParseFail))?;

    match response {
        model::PapagoResponse::Success { message } => Ok(message.result.translated_text),
        model::PapagoResponse::Fail {
            error_message,
            error_code: _error_code,
        } => Err(Error::PapagoError(error_message)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[test]
    #[allow(non_snake_case)]
    fn test_send_NMT_request() {
        let auth = config::get_auth_from_env().unwrap();
        let payload = model::PapagoReq {
            source: model::Lang::En,
            target: model::Lang::Ko,
            text: "HELLO WORLD".to_owned(),
        };
        let result = send_request(NMT_URL, &auth, payload);
        assert_eq!(result.unwrap(), "안녕 세계".to_owned());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_wrong_NMT_request__same_source_target() {
        let auth = config::get_auth_from_env().unwrap();
        let payload = model::PapagoReq {
            source: model::Lang::En,
            target: model::Lang::En,
            text: "HELLO WORLD".to_owned(),
        };
        let result = send_request(NMT_URL, &auth, payload);
        assert_eq!(
            result.unwrap_err(),
            Error::PapagoError(
                "source and target must be different (source와 target이 동일합니다.)"
                    .to_owned()
            )
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_wrong_NMT_request__wrong_lang_combo() {
        let auth = config::get_auth_from_env().unwrap();
        let payload = model::PapagoReq {
            source: model::Lang::Vi,
            target: model::Lang::Es,
            text: "HELLO WORLD".to_owned(),
        };
        let result = send_request(NMT_URL, &auth, payload);
        assert_eq!(
            result.unwrap_err(),
            Error::PapagoError(
"There is no source-to-target translator (source->target 번역기가 없습니다.)"
                    .to_owned()
            )
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_send_SMT_request() {
        let auth = config::get_auth_from_env().unwrap();
        let payload = model::PapagoReq {
            source: model::Lang::En,
            target: model::Lang::Ko,
            text: "HELLO WORLD".to_owned(),
        };
        let result = send_request(SMT_URL, &auth, payload);
        assert_eq!(result.unwrap(), "안녕 세계".to_owned());
    }
}
