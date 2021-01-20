use std::fmt::Display;

pub mod factory;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CexObject(serde_json::Value);
impl CexObject {
    pub fn new(cex_json: &str) -> Result<Self, ()> {
        serde_json::from_str(cex_json)
            .map(|value| Self(value))
            .map_err(|_err| ())
    }

    pub fn e<'a>(&'a self) -> Option<&'a Event> {
        self.0
            .get("e")
            .map(|e| e.as_str())
            .flatten()
    }

    pub fn is_ok(&self) -> bool {
        self.0["ok"].as_str()
            .map(|ok| ok == "ok")
            .unwrap_or(false)
    }

    pub fn is_error(&self) -> bool {
        self.0["ok"].as_str()
            .map(|ok| ok == "error")
            .unwrap_or(false)
    }
}

// impl Into<tungstenite::Message> for CexObject {
//     fn into(self) -> tungstenite::Message {
//         tungstenite::Message::Text(self.0.to_string())
//     }
// }

pub type Event = str;
// Protocol messages
pub const CONNECTED: &Event = "connected";
pub const PING: &Event = "ping";
pub const PONG: &Event = "pong";
pub const DISCONNECTING: &Event = "disconnecting";
// Public channels
pub const AUTH: &Event = "auth";
pub const SUBSCRIBE: &Event = "subscribe";
pub const TICK: &Event = "tick";
pub const MD: &Event = "md";
pub const MD_GROUPPED: &Event = "md_groupped";
pub const HISTORY: &Event = "history";
pub const HISTORY_UPDATE: &Event = "history-update";
pub const INIT_OHLCV: &Event = "init-ohlcv";
pub const OHLCV: &Event = "ohlcv";
pub const OHLCV24: &Event = "ohlcv24";
pub const INIT_OHLCV_DATA: &Event = "init-ohlcv-data";
pub const OHLCV1M: &Event = "ohlcv1m";
pub const OPEN_ORDERS: &Event = "open-orders";
// Private Channels
pub const TICKER: &Event = "ticker";
pub const GET_BALANCE: &Event = "get-balance";
pub const ORDER_BOOK_SUBSCRIBE: &Event = "order-book-subscribe";
pub const ORDER_BOOK_UNSUBSCRIBE: &Event = "order-book-unsubscribe";
pub const PLACE_ORDER: &Event = "place-order";
pub const CANCEL_REPLACE_ORDER: &Event = "cancel-replace-order";
pub const GET_ORDER: &Event = "get-order";
pub const CANCEL_ORDER: &Event = "cancel-order";
pub const ARCHIVED_ORDERS: &Event = "archived-orders";
// Subscription messages
pub const TX: &Event = "tx";
pub const BALANCE: &Event = "balance";
pub const OBALANCE: &Event = "obalance";
pub const MD_UPDATE: &Event = "md_update";
pub const ORDER: &Event = "order";

impl Display for CexObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_events() {
        let o1 = CexObject::new(r#"{"e":"md"}"#).unwrap();
        assert_eq!(o1.e(), Some("md"));

        let o3 = CexObject(json!({"e": {"isnt": "string"}}));
        assert_eq!(o3.e(), None);

        let o4 = CexObject(json!({"no": "event"}));
        assert_eq!(o4.e(), None);
    }

    #[test]
    fn test_ok_error() {
        let ok = CexObject(json!({"ok": "ok"}));
        assert_eq!(ok.is_ok(), true);
        assert_eq!(ok.is_error(), false);

        let error = CexObject(json!({"ok": "error"}));
        assert_eq!(error.is_ok(), false);
        assert_eq!(error.is_error(), true);

        let neither = CexObject(json!({"foo": "bar"}));
        assert_eq!(neither.is_ok(), false);
        assert_eq!(neither.is_error(), false);
    }
}
