use std::env;

extern crate reqwest;
extern crate serde;
extern crate ws;
#[macro_use]
extern crate serde_derive;
// Response { url: "https://slack.com/api/rtm.connect?mpim_aware=1", status: 200, headers: {"content-type": "application/json; charset=utf-8", "via": "1.1 77ad74c5c38202142d837f12e773d252.cloudfront.net (CloudFront)", "connection": "keep-alive", "date": "Thu, 20 Sep 2018 15:23:01 GMT", "server": "Apache", "x-accepted-oauth-scopes": "rtm:stream,client", "x-content-type-options": "nosniff", "x-slack-router": "p", "expires": "Mon, 26 Jul 1997 05:00:00 GMT", "cache-control": "private, no-cache, no-store, must-revalidate", "x-oauth-scopes": "identify,read,post,client,apps", "pragma": "no-cache", "x-xss-protection": "0", "x-slack-req-id": "ce176cf3-70f6-431e-850d-6fa32837b2ad", "x-slack-exp": "1", "x-slack-backend": "h", "referrer-policy": "no-referrer", "strict-transport-security": "max-age=31536000; includeSubDomains; preload", "vary": "Accept-Encoding", "x-amz-cf-id": "1YzTi3j8VJzZhEJfECWWZ69CCm1K5c9jSCHQmn_eh_54IYfbhcKE0A==", "access-control-allow-origin": "*", "x-via": "haproxy-www-i8ac", "x-cache": "Miss from cloudfront"}
// Ok("{\"ok\":true,\"url\":\"wss:\\/\\/cerberus-xxxx.lb.slack-msgs.com\\/websocket\\/bwnMd908ecKcp6YJNooHI25gM2kShLC6x-3bQKLIuyksrUcVqtStpfQaIc5fWMSvu5Yk8_9MMej2lY4CzpkF8i-tTTwpYqZEe_MIFpk-VVQ=\",\"team\":{\"id\":\"T100W6508\",\"name\":\"Rebase, Inc.\",\"domain\":\"rebase-team\"},\"self\":{\"id\":\"U1013370U\",\"name\":\"yuya373\"}}")
#[derive(Debug, Deserialize)]
struct ConnectResponse {
    ok: bool,
    url: String,
    team: Team,
    #[serde(rename = "self")]
    me: Me,
}
#[derive(Debug, Deserialize)]
struct Team {
    id: String,
    name: String,
    domain: String,
}
#[derive(Debug, Deserialize)]
struct Me {
    id: String,
    name: String,
}

fn connect(token: &str, client: &reqwest::Client) -> reqwest::Result<reqwest::Response> {
    let url = "https://slack.com/api/rtm.connect";
    let builder = client.get(url);
    let query = [("mpim_aware", "1")];

    builder.bearer_auth(token).query(&query).send()
}

fn main() {
    let token = env::var("TOKEN").expect("TOKEN='xoxp-foo...'");
    let client = reqwest::Client::new();

    let response: ConnectResponse = connect(&token, &client).unwrap().json().unwrap();

    println!("connect to: {:?}", response.url);
    ws::connect(response.url, |out| {
        move |msg| {
            println!("Message: {}", msg);
            out.close(ws::CloseCode::Normal)
        }
    }).unwrap()
}
