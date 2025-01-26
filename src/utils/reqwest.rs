use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;


struct ReqResult<T> {
    code: Arc<u16>,
    body: Arc<T>,
}

async fn request<U, T> (
    url: String, 
    method: reqwest::Method, 
    body: &U,
    key: String
) -> Result<T, u16>
where
    T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug,
{ 
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut req = reqwest::Client::new()
        .request(method, url)
        .bearer_auth(key)
        .header("Content-Type", "application/json");
    if allow_body { 
        req = req.json(body);
    }
    println!("=============");
    println!("Req: {:?}", req);
    let res_resp = req.send().await;
    println!("Resp: {:?}", res_resp);
    println!("=============");
    match res_resp {
        Ok(resp) => {

        match resp.status().is_success(){
            true => {
                match resp.json::<T>().await{
                    Ok(data) => Ok(data),
                    Err(_) => {
                        println!("Failed parse body");
                        Err(0)
                    },
                }
            },
            false => {
                Err(resp.status().as_u16())
            }
        }
    },
        Err(err) => {
            println!("err: {:?}", err);
            Err(0)
        }
    }
}

pub async fn request_delete<T>(url: String, key: String) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::DELETE, &(), key).await
}

pub async fn request_get<T>(url: String, key: String) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::GET, &(), key).await
}

pub async fn request_post<U, T>(url: String, body: &U, key: String) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
{
    request(url, reqwest::Method::POST, body, key).await
}

pub async fn request_put<U, T>(url: String, body: &U, key: String) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
{
    request(url, reqwest::Method::PUT, body, key).await
}