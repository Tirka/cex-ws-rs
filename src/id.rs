use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

pub struct CexID {
    api_key: String,
    api_secret: String
}

impl CexID {
    pub fn new<S: ToString>(api_key: S, api_secret: S) -> Self {
        Self {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string()
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn calculate_signature(&self, unix_time: u64) -> String {
        type HmacSha256 = Hmac<Sha256>;
        let mut hmac = HmacSha256::new_varkey(self.api_secret.as_bytes()).unwrap();
        hmac.update(format!("{}{}", unix_time, self.api_key).as_bytes());
        hmac.finalize()
            .into_bytes()
            .as_slice()
            .into_iter()
            .fold(String::with_capacity(64), |mut acc, next| {
                acc.push_str(&format!("{:02x?}", next));
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::CexID;

    // test api tokens and signatures found here https://cex.io/websocket-api-dsl
    static TEST_API_KEY: &str = "1WZbtMTbMbo2NsW12vOz9IuPM";
    static TEST_API_SECRET: &str = "1IuUeW4IEWatK87zBTENHj1T17s";

    #[test]
    fn test_digest() {
        let test_pairs = [
            (1448034533u64, "7d581adb01ad22f1ed38e1159a7f08ac5d83906ae1a42fe17e7d977786fe9694"),
            (1448035135u64, "9a84b70f51ea2b149e71ef2436752a1a7c514f521e886700bcadd88f1767b7db")
        ];

        let cex_id = CexID::new(TEST_API_KEY, TEST_API_SECRET);

        for (timestamp, digest) in test_pairs.iter() {
            assert_eq!(
                cex_id.calculate_signature(*timestamp),
                *digest
            )
        }
    }
}
