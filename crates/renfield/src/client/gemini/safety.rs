//! Safety & moderation data types.

use serde::{Serialize, Deserialize};

/// Safety setting, affecting the safety-blocking behavior.
///
/// Passing a safety setting for a category changes the allowed
/// probability that content is blocked.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#safetysetting)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafetySettings {

    /// The category for this setting.
    pub category: HarmCategory,

    /// Controls the probability threshold at which harm is blocked.
    pub threshold: HarmBlockThreshold,
}

impl SafetySettings {
    pub fn new(category: HarmCategory, threshold: HarmBlockThreshold) -> Self {
        Self { category, threshold }
    }
}

/// The category of a rating.
///
/// These categories cover various kinds of harms that developers may
/// wish to adjust.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#harmcategory)
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum HarmCategory {

    /// Negative or harmful comments targeting identity and/or
    /// protected attribute.
    ///
    /// Only available for PaLM models.
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    Derogatory,

    /// Content that is rude, disrespectful, or profane.
    ///
    /// Only available for PaLM models.
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    Toxicity,

    /// Describes scenarios depicting violence against an individual
    /// or group, or general descriptions of gore.
    ///
    /// Only available for PaLM models.
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    Violence,

    /// Contains references to sexual acts or other lewd content.
    ///
    /// Only available for PaLM models.
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    Sexual,

    /// Promotes unchecked medical advice.
    ///
    /// Only available for PaLM models.
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    Medical,

    /// Dangerous content that promotes, facilitates, or encourages
    /// harmful acts.
    ///
    /// Only available for PaLM models.
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    Dangerous,

    /// Harassment content.
    ///
    /// Only available on Gemini-series models.
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    Harassment,

    /// Hate speech and content.
    ///
    /// Only available on Gemini-series models.
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HateSpeech,

    /// Sexually explicit content.
    ///
    /// Only available on Gemini-series models.
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    SexuallyExplicit,

    /// Dangerous content.
    ///
    /// Only available on Gemini-series models.
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    DangerousContent,

    /// Content that may be used to harm civic integrity.
    ///
    /// Only available on Gemini-series models.
    #[serde(rename = "HARM_CATEGORY_CIVIC_INTEGRITY")]
    CivicIntegrity,
}

/// Block at and beyond a specified harm probability.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#HarmBlockThreshold)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {

    /// Content with NEGLIGIBLE will be allowed.
    BlockLowAndAbove,

    /// Content with NEGLIGIBLE and LOW will be allowed.
    BlockMediumAndAbove,

    /// Content with NEGLIGIBLE, LOW, and MEDIUM will be allowed.
    BlockOnlyHigh,

    /// All content will be allowed.
    BlockNone,

    /// Turn off the safety filter.
    Off,
}

/// Safety rating for a piece of content.
///
/// The safety rating contains the category of harm and the harm
/// probability level in that category for a piece of content. Content
/// is classified for safety across a number of harm categories and
/// the probability of the harm classification is included here.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#safetyrating)
#[derive(Deserialize, Debug, Clone)]
pub struct SafetyRating {

    /// The category for this rating.
    pub category: HarmCategory,

    /// The probability of harm for this content.
    pub probability: HarmProbability,

    /// Was this content blocked because of this rating?
    pub blocked: Option<bool>,
}

/// The probability that a piece of content is harmful.
///
/// The classification system gives the probability of the content
/// being unsafe. This does not indicate the severity of harm for a
/// piece of content.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {

    /// Content has a negligible chance of being unsafe.
    Negligible,

    /// Content has a low chance of being unsafe.
    Low,

    /// Content has a medium chance of being unsafe.
    Medium,

    /// Content has a high chance of being unsafe.
    High,
}
