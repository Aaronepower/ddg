use super::*;

/// The Response from DuckDuckGo.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct Response {
    /// The Topic summary (can contain HTML, e.g. italics).
    #[serde(rename="Abstract")]
    pub abstract_html: String,
    /// The Topic summary with no HTML.
    #[serde(rename="AbstractText")]
    pub abstract_text: String,
    /// name of the source of the abstract.
    #[serde(rename="AbstractSource")]
    pub abstract_source: String,
    /// A deep link to the expanded topic page from `abstract_source`.
    #[serde(rename="AbstractURL")]
    pub abstract_url: String,
    /// A link to a image that goes with the abstract.
    #[serde(rename="Image")]
    pub image: String,
    /// The name of the topic that goes with the abstract.
    #[serde(rename="Heading")]
    pub heading: String,
    /// The Instant Answer.
    #[serde(rename="Answer")]
    pub answer: String,
    /// type of Answer, e.g. calc, color, digest, info, ip, iploc, phone, pw,
    /// rand, regexp, unicode, upc, or zip, etc. (See
    /// [tour page](https://duckduckgo.com/tour) for examples)
    #[serde(rename="AnswerType")]
    pub answer_type: String,
    /// The dictionary definition which **may differ from the abstract**.
    #[serde(rename="Definition")]
    pub definition: String,
    /// The name of the source of the dictionary definition.
    #[serde(rename="DefinitionSource")]
    pub definition_source: String,
    /// The url to the definition's source.
    #[serde(rename="DefinitionURL")]
    pub definition_url: String,
    /// A vec of internal links to related topics associated with the abstract.
    #[serde(rename="RelatedTopics")]
    pub related_topics: Vec<RelatedTopic>,
    /// A vec of external links associated with the abstract.
    #[serde(rename="Results")]
    pub results: Vec<TopicResult>,
    /// The response's category, i.e. A (article), D (disambiguation), C (category),
    /// N (name), E (exclusive), or nothing.
    #[serde(rename="Type")]
    pub response_type: Type,
    /// Redirect URL from a !bang query.
    #[serde(rename="Redirect")]
    pub redirect: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn is_valid_json() {
        let json = include_str!("../../tests/test.json");

        let actual_json: Response = serde_json::from_str(json).unwrap();

        assert_eq!(actual_json.heading, "Apple");
    }
}
