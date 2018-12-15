/// Lang is a target/source language
/// For more information, check [official doc](https://developers.naver.com/docs/nmt/reference/)
#[derive(Serialize, Deserialize)]
pub enum Lang {
    /// Korean
    #[serde(rename = "ko")]
    Ko,
    /// English
    #[serde(rename = "en")]
    En,
    /// Simplified Chinese
    #[serde(rename = "cn")]
    Cn,
    /// Traditional Chinese
    #[serde(rename = "tw")]
    Tw,
    /// Spainish
    #[serde(rename = "es")]
    Es,
    /// French
    #[serde(rename = "fr")]
    Fr,
    /// Vietnamnese
    #[serde(rename = "vi")]
    Vi,
    /// Thai
    #[serde(rename = "th")]
    Th,
    /// Indonesia
    #[serde(rename = "id")]
    Id,
}

/// PapagoReq is the payload sent to Naver Papago API
/// It has to be sent as a form request (`x-www-form-urlencoded`)
#[derive(Serialize, Deserialize)]
pub struct PapagoReq {
    /// Source Language
    pub source: Lang,
    /// Target Language
    pub target: Lang,
    /// Text in Source Language
    pub text: String,
}


/// Message is an intermediate type of the response
/// You won't need this
#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@service")]
    pub _service: String,
    #[serde(rename = "@version")]
    pub _version: String,
    pub result: PapagoResult,
}

/// PapagoResult contains the translation
#[derive(Serialize, Deserialize)]
pub struct PapagoResult {
    #[serde(rename = "translatedText")]
    pub translated_text: String,
}

/// PapagoResponse is the response sent by Papago
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PapagoResponse {
    Success {
        message: Message,
    },
    Fail {
        #[serde(rename = "errorMessage")]
        error_message: String,
        #[serde(rename = "errorCode")]
        error_code: String,
    },
}
