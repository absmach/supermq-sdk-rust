#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessagesPageMessagesInner {
    /// Unique channel id.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    /// Unique publisher id.
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<i32>,
    /// Protocol name.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Measured parameter name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value unit.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Measured value in number.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
    /// Measured value in string format.
    #[serde(rename = "stringValue", skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    /// Measured value in boolean format.
    #[serde(rename = "boolValue", skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
    /// Measured value in binary format.
    #[serde(rename = "dataValue", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    /// Sum value.
    #[serde(rename = "valueSum", skip_serializing_if = "Option::is_none")]
    pub value_sum: Option<f32>,
    /// Time of measurement.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<f32>,
    /// Time of updating measurement.
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessagesPage {
    /// Total number of items that are present on the system.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
    /// Number of items that were skipped during retrieval.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<f32>,
    /// Size of the subset that was retrieved.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f32>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::MessagesPageMessagesInner>>,
}

impl MessagesPage {
    pub fn new() -> MessagesPage {
        MessagesPage {
            total: None,
            offset: None,
            limit: None,
            messages: None,
        }
    }
}


impl MessagesPageMessagesInner {
    pub fn new() -> MessagesPageMessagesInner {
        MessagesPageMessagesInner {
            channel: None,
            publisher: None,
            protocol: None,
            name: None,
            unit: None,
            value: None,
            string_value: None,
            bool_value: None,
            data_value: None,
            value_sum: None,
            time: None,
            update_time: None,
        }
    }
}