use serde::{Deserialize, Serialize};

/// This object contains basic information about an invoice.
///
/// [The official docs](https://core.telegram.org/bots/api#invoice).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Invoice {
    /// Product name.
    pub title: String,

    /// Product description.
    pub description: String,

    /// Unique bot deep-linking parameter that can be used to generate this
    /// invoice.
    pub start_parameter: String,

    /// Three-letter ISO 4217 currency code.
    pub currency: String,

    /// Total price in the smallest units of the currency (integer, **not**
    /// float/double). For example, for a price of `US$ 1.45` pass `amount =
    /// 145`. See the exp parameter in [`currencies.json`], it shows the number
    /// of digits past the decimal point for each currency (2 for the
    /// majority of currencies).
    ///
    /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: i32,
}

impl Invoice {
    pub fn new<S1, S2, S3, S4>(
        title: S1,
        description: S2,
        start_parameter: S3,
        currency: S4,
        total_amount: i32,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            title: title.into(),
            description: description.into(),
            start_parameter: start_parameter.into(),
            currency: currency.into(),
            total_amount,
        }
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = val.into();
        self
    }

    pub fn start_parameter<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.start_parameter = val.into();
        self
    }

    pub fn currency<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.currency = val.into();
        self
    }

    pub fn total_amount(mut self, val: i32) -> Self {
        self.total_amount = val;
        self
    }
}
