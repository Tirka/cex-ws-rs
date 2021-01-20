use serde_json::json;

use crate::id::CexID;
use super::CexObject;

fn unixtime_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn auth_request(cex_id: &CexID) -> CexObject {
    let time = unixtime_now();
    CexObject(json!({
        "e": "auth",
        "auth": {
            "key": cex_id.api_key(),
            "signature": cex_id.calculate_signature(time),
            "timestamp": time
        }
    }))
}

pub fn ticker(p1: &str, p2: &str) -> CexObject {
    let mut ticker = json!({
        "e": "ticker",
        "data": []
    });

    let data = ticker["data"].as_array_mut().unwrap();

    data.push(p1.into());
    data.push(p2.into());

    CexObject(ticker)
}

pub fn pong() -> CexObject {
    CexObject(json!({"e": "pong"}))
}

pub fn get_balance() -> CexObject {
    CexObject(json!({
        "e": "get-balance"
    }))
}

pub fn open_orders(p1: &str, p2: &str) -> CexObject {
    let mut orders = json!({
        "e": "open-orders",
        "data": {
            "pair": []
        }
    });

    let pair = orders["data"]["pair"].as_array_mut().unwrap();

    pair.push(p1.into());
    pair.push(p2.into());

    CexObject(orders)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    #[test]
    fn test_ticker() {
        assert_eq!(
            super::ticker("BTC", "USD").0,
            json!({
                "e": "ticker",
                "data": ["BTC", "USD"]
            })
        )
    }

    #[test]
    fn test_orders() {
        assert_eq!(
            super::open_orders("BTC", "USD").0,
            json!({
                "e": "open-orders",
                "data": {
                    "pair": ["BTC", "USD"]
                }
            })
        )
    }
}
