use anyhow::Error;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use web_sys::console::log_1;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/backend/schema.graphql",
    query_path = "src/backend/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct RepoView;

async fn get_data() -> Result<(), Error> {
    let github_token = std::env::var("GITHUB_API_TOKEN").expect("Github api token not found!");
    let (owner, repo) = ("lunchspider", "quark");
    let variables = repo_view::Variables {
        owner: owner.into(),
        name: repo.into(),
    };
    let request_body = RepoView::build_query(variables);
    let client = Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;
    let res = client.post("https://api.github.com/graphql").json(&request_body).send().await?;
    let response_body: Response<repo_view::ResponseData> = res.json().await?;
    log_1(&format!("{:?}", response_body).into());
    Ok(())
}
