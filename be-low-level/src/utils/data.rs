use redis::Commands;
use std::error::Error;
use std::sync::{Arc, Mutex};

struct RedisClient {
    con: redis::Connection,
}

impl RedisClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let con = client.get_connection()?;
        Ok(RedisClient { con })
    }

    pub fn add_token_r(&mut self, token: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let _: () = self.con.hset("signedTokens", token, value)?;
        Ok(())
    }

    pub fn remove_token_r(&mut self, token: &str) -> Result<(), Box<dyn Error>> {
        let _: () = self.con.hdel("signedTokens", token)?;
        Ok(())
    }

    pub fn get_token_r(&mut self, token: &str) -> Result<Option<String>, Box<dyn Error>> {
        let value: Option<String> = self.con.hget("signedTokens", token)?;
        Ok(value)
    }

    pub fn get_wins_r(&mut self) -> Result<Option<u32>, Box<dyn Error>> {
        let value: Option<String> = self.con.get("totalWins")?;
        let wins = value.map(|v| v.parse::<u32>().unwrap_or(0));
        Ok(wins)
    }

    pub fn update_win_r(&mut self, token: &str, did_win: bool) -> Result<(), Box<dyn Error>> {
        let token_data: String = self.get_token_r(token)?.ok_or("Token not found")?;

        let mut data: serde_json::Value = serde_json::from_str(&token_data)?;
        let wins = data["wins"].as_u64().unwrap_or(0);
        let losses = data["losses"].as_u64().unwrap_or(0);

        if did_win {
            data["wins"] = serde_json::json!(wins + 1);
            let _: () = self.con.set("totalWins", wins + 1)?;
        } else {
            data["losses"] = serde_json::json!(losses + 1);
        }

        self.add_token_r(token, &data.to_string())?;

        Ok(())
    }
}

lazy_static::lazy_static! {
    static ref REDIS_CLIENT: Arc<Mutex<RedisClient>> = {
        let client = RedisClient::new().expect("Failed to create Redis client");
        Arc::new(Mutex::new(client))
    };
}

pub async fn add_token(token: &str) -> bool {
    let client = REDIS_CLIENT.lock();
    client.expect("REASON").add_token_r(token, r#"{ "wins": 0, "losses": 0 }"#).expect("Failed to add token");
    true
}

pub async fn remove_token(token: &str) -> bool {
    let client = REDIS_CLIENT.lock();
    client.expect("REASON").remove_token_r(token).expect("Failed to remove token");
    true
}

pub async fn get_total_wins() -> Result<i64, Box<dyn std::error::Error>> {
    let client = REDIS_CLIENT.lock();

    match client?.get_wins_r() {
        Ok(Some(wins)) => Ok(wins as i64),
        Ok(None) => Ok(0),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_win(token: &str, did_win: bool) -> bool {
    let client = REDIS_CLIENT.lock();
    client.expect("REASON").update_win_r(token, did_win).expect("Failed to update win");
    true
}
