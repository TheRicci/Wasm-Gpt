use leptos::*;
use crate::model::chatlog::ChatLog;

#[server(Request "/api")]
pub async fn request(cx: Scope, prompt: ChatLog) -> Result<String, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;
    use openai_api_rs::v1::api::Client;
    use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

    let client = extract(cx, |data: Data<Client>, _connection: ConnectionInfo| async {
        data.into_inner()
    })
    .await.unwrap();

    let req = ChatCompletionRequest {
        model: chat_completion::GPT3_5_TURBO.to_string(),
        messages: prompt.chat.iter().map(|line|{
            let role: chat_completion::MessageRole;
            if line.user {
                role = chat_completion::MessageRole::user
            }else{
                role = chat_completion::MessageRole::assistant
            }
            chat_completion::ChatCompletionMessage {
                role: role,
                content: line.message.clone(),
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
    let result = client.chat_completion(req).await;
    let res = match result {
        Ok(r) => r,
        Err(y) => return Err(ServerFnError::ServerError(y.message.to_string().clone())),
    };

    Ok(res.choices[0].message.content.clone().unwrap())
}

