/// http request mod
/// execute get or post, return response
pub mod my_request {
    use reqwest::Error;
    use serde::{Deserialize, Serialize};
    use serde_json::Number;
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    struct ResBody {
        code: String,
        data: Token,
    }

    /// token struct
    #[derive(Serialize, Deserialize)]
    struct Token {
        token: String,
        expire: Number,
    }

    /// get token
    pub async fn get_token() -> Result<String, Error> {
        let base_uri = "https://*********";
        let auth_url = base_uri.to_owned() + "/api/auth/*********";
        let mut map = HashMap::new();
        map.insert("appCode", "*********");
        map.insert("appSecret", "*********");
        let response_txt = do_post(auth_url, map).await?;
        let result: ResBody = serde_json::from_str(response_txt.as_str()).unwrap();
        let token = result.data.token;
        println!("token=>{:#?}", token);
        Ok(token)
    }

    /// do get
    pub async fn do_get(uri: &str) -> Result<String, Error> {
        let body = reqwest::get(uri)
            .await?
            .text()
            .await?;
        println!("body = {body:?}");
        Ok(body)
    }


    /// do post and get response
    pub async fn do_post(uri: String, body: HashMap<&str, &str>) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client.post(uri)
            .json(&body)
            .send()
            .await?;
        let response_txt = response.text().await?;
        println!("response = {response_txt:?}");
        Ok(response_txt)
    }
}

