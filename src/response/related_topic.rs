use std::fmt;

use super::Icon;
use serde::de;

/// Internal link(s) to related topics associated with abstract. A result could
/// either be a single `TopicResult`, or a `Topic` containing multiple
/// `TopicResult`s in a certain area of interest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelatedTopic {
    /// The link had a single topic.
    TopicResult(TopicResult),
    /// The link had a whole topic category.
    Topic(Topic),
}

impl<'de> de::Deserialize<'de> for RelatedTopic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        deserializer.deserialize_map(Visitor)
    }
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = RelatedTopic;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a object containing with Topics, FirstURL, Result properties")
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<RelatedTopic, V::Error>
        where V: de::MapAccess<'de>
    {
        let s: String = visitor.next_key()?.expect("got struct with no fields");
        let val = match &*s {
            "Topics" => {
                Ok(RelatedTopic::Topic(Topic {
                    topics: visitor.next_value()?,
                    name: {
                        let s: String = visitor.next_key()?.expect("Name field");
                        assert_eq!(&s, "Name");
                        visitor.next_value()?
                    },
                }))
            }
            "FirstURL" => {
                Ok(RelatedTopic::TopicResult(TopicResult {
                    first_url: visitor.next_value()?,
                    icon: {
                        let s: String = visitor.next_key()?.expect("icon field");
                        assert_eq!(&s, "Icon");
                        visitor.next_value()?
                    },
                    result: {
                        let s: String = visitor.next_key()?.expect("result field");
                        assert_eq!(&s, "Result");
                        visitor.next_value()?
                    },
                    text: {
                        let s: String = visitor.next_key()?.expect("text field");
                        assert_eq!(&s, "Text");
                        visitor.next_value()?
                    },
                }))
            },
            "Result" => {
                Ok(RelatedTopic::TopicResult(TopicResult {
                    result: visitor.next_value()?,
                    icon: {
                        let s: String = visitor.next_key()?.expect("icon field");
                        assert_eq!(&s, "Icon");
                        visitor.next_value()?
                    },
                    first_url: {
                        let s: String = visitor.next_key()?.expect("result field");
                        assert_eq!(&s, "FirstURL");
                        visitor.next_value()?
                    },
                    text: {
                        let s: String = visitor.next_key()?.expect("text field");
                        assert_eq!(&s, "Text");
                        visitor.next_value()?
                    },
                }))
            },
            other => panic!("no struct has field `{}`", other),
        };
        val
    }
}

/// An link associated with abstract.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TopicResult {
    /// First URL for the Result.
    #[serde(rename="FirstURL")]
    pub first_url: String,
    /// Icon associated with `first_url`.
    #[serde(rename="Icon")]
    pub icon: Icon,
    /// HTML link(s) to external site(s).
    #[serde(rename="Result")]
    pub result: String,
    /// Text from `first_url`.
    #[serde(rename="Text")]
    pub text: String,
}


/// Name, and Vec of `TopicResult`s.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Topic {
    /// vec of external links associated with abstract.
    #[serde(rename="Topics")]
    pub topics: Vec<RelatedTopic>,
    /// The name of the topic. for example if the query was Apple, it could be
    /// a topic about botany, or companies.
    #[serde(rename="Name")]
    pub name: String,
}
