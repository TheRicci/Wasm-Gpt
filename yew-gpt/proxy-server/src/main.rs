use rocket::{post, serde::json::Json,State,response::status::Custom,http::Status};
use serde::{Serialize,Deserialize};
use std::sync::Arc;
use dotenv;
use rocket::http::Header;
use rocket::{Request, Response,launch,routes};
use rocket::fairing::{Fairing, Info, Kind};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

#[derive(Serialize, Debug)]
struct Data {
    message: String,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize, Debug)]
struct OpenAIResponse {
    status: String,
    data: Data,
}

#[derive(Serialize)]
struct GenericResponse {
    status: String,
    message: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Input{
    messanger: String,
    message: String
}

#[derive(Debug, Deserialize, Clone)]
struct Message {
    inputs: Vec<Input>,
}

#[post("/request", data = "<body>")]
async fn openai_request_handler(
    body: Json<Message>,
    data: &State<AppState>,
) -> Result<Json<OpenAIResponse>, Custom<Json<GenericResponse>>> {
    let req = ChatCompletionRequest {
        model: chat_completion::GPT3_5_TURBO.to_string(),
        messages: body.inputs.iter().map(|line|{
            let role: chat_completion::MessageRole;
            if line.messanger == "user"{
                role = chat_completion::MessageRole::user
            }else{
                role = chat_completion::MessageRole::assistant
            }
            
            chat_completion::ChatCompletionMessage {
                role: role,
                content: Some(line.message.clone()),
                name: None,
                function_call: None,
            }
    
            }).collect(),
        functions: None,
        function_call: None,
        temperature: None,
        top_p: None,
        n: None,
        stream: None,
        stop: None,
        max_tokens: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };
    let result = data.openai_client.chat_completion(req).await;
    let res = match result {
        Ok(r) => r,
        Err(y) => return Err(Custom(Status::InternalServerError, Json(GenericResponse { status: "fail".to_owned(), message: y.message.to_string().clone() }))),
    };
 
    Ok(Json(OpenAIResponse{status: "success".to_string(),data: Data{message: res.choices[0].message.content.clone().unwrap()} }))
}

struct AppState {
    openai_client: Arc<openai_api_rs::v1::api::Client>,
}

impl AppState {
   fn init(api_token:String) -> AppState {
        AppState {
            openai_client: Arc::new(Client::new(api_token))
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let app_data = AppState::init(std::env::var("OPENAI_KEY").unwrap());
    rocket::build().manage(app_data).attach(CORS)
    .mount(
        "/",
        routes![
            openai_request_handler,
        ],
    )
}
