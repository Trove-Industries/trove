use reqwest::Client;
use std::env;

#[derive(Clone)]
pub struct SupabaseClient {
    pub http: Client,
    pub base_url: String,
    pub api_key: String,
}

impl SupabaseClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let base_url = env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
        let api_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY not set");

        Self {
            http: Client::new(),
            base_url,
            api_key,
        }
    }

    // TODO create a model for <serde_json::Value>
    pub async fn signup_user(
        &self,
        email: &String,
        password: &String,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/auth/v1/signup", self.base_url);
        let res = self
            .http
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "email": email, "password": password }))
            .send()
            .await?;

        res.json::<serde_json::Value>().await
    }

    pub async fn login_user(
        &self,
        email: &String,
        password: &String,
    )->Result<serde_json::Value, reqwest::Error>{
        let url = format!("{}/auth/v1/token?grant_type=password", self.base_url);
        let res = self
            .http
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({"email": email, "password": password}))
            .send()
            .await?;
        res.json::<serde_json::Value>().await
    }
}
