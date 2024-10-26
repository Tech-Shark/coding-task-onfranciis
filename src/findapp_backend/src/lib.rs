use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_cdk::update;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

// This struct is legacy code and is not really used in the code.
#[derive(Serialize, Deserialize)]
struct Context {
    bucket_start_time_index: usize,
    closing_price_index: usize,
}

#[update]
async fn send(
    purpose: String,
    timeframe: String,
    totalamount: String,
    monthlyincome: String,
    monthlyexpenses: String,
) -> String {
    ic_cdk::api::print(format!(
        "Received {purpose} \n {timeframe} \n
    {totalamount} \n {monthlyincome} \n {monthlyexpenses}"
    ));

    send_post_request(
        purpose,
        timeframe,
        totalamount,
        monthlyincome,
        monthlyexpenses,
    )
    .await
}

#[update]
async fn send_post_request(
    purpose: String,
    timeframe: String,
    totalamount: String,
    monthlyincome: String,
    monthlyexpenses: String,
) -> String {
    let url = "https://icp-api-budget.fly.dev/budget-plan";
    let host = "icp-api-budget.fly.dev";

    let headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "demo_HTTP_POST_canister".to_string(),
        },
        HttpHeader {
            name: "Idempotency-Key".to_string(),
            value: "UUID-123456789".to_string(),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let body = Some(
        json!({
        "purpose": purpose ,
        "timeframe":timeframe ,
        "totalAmount": totalamount,
        "totalIncomeMonthly": monthlyincome,
        "totalExpensesMonthly":monthlyexpenses
         })
        .to_string()
        .into_bytes(),
    );

    let context = Context {
        bucket_start_time_index: 0,
        closing_price_index: 4,
    };

    ic_cdk::api::print(format!("About to post {:#?}", body.clone()));

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: None, //optional for request
        method: HttpMethod::POST,
        headers,
        body,
        transform: Some(TransformContext::from_name(
            "transform".to_string(),
            serde_json::to_vec(&context).unwrap(),
        )),
    };

    // Make the POST request
    let result = http_request(request, 2_603_184_000).await;

    // Handle the response
    match result {
        Ok(response) => {
            //if successful, `HttpResponse` has this structure:
            // pub struct HttpResponse {
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            //You need to decode that Vec<u8> that is the body into readable text.
            //To do this:
            //  1. Call `String::from_utf8()` on response.body
            //  3. Use a switch to explicitly call out both cases of decoding the Blob into ?Text
            let str_body = String::from_utf8(response.0.body)
                .expect("Transformed response is not UTF-8 encoded.");
            ic_cdk::api::print(format!("{:?}", str_body));

            //Return the body as a string and end the method
            let result: String = format!("{str_body}");

            format!("{:#?}", result)
        }
        Err((code, string)) => {
            ic_cdk::api::print(format!("Error: code - {:?} \n {:?} ", code, string));

            format!("Error: code - {:?} \n {:?} ", code, string)
        }
    }
}

// Strips all data that is not needed from the original response.
#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    if res.status == (200 as u64) {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:#?}", raw));
    }

    res
}
