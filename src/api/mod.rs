use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery};
use helpers::{ClientBuilder, Headers};
use reqwest::header::HeaderMap;
use serde::Serialize;
use user_info::UserInfoUser;

const API_URL: &str = "https://api.github.com/graphql";

#[derive(GraphQLQuery)]
#[graphql(query_path = "query.graphql", schema_path = "schema.graphql")]
struct UserInfo;

impl UserInfoUser {
    const fn normalize(self) -> Info {
        Info {
            pull_requests: self.pull_requests.total_count,
            issues: self.issues.total_count,
            followers: self.followers.total_count,
        }
    }
}

#[derive(Serialize)]
pub struct Info {
    pull_requests: i64,
    issues: i64,
    followers: i64,
}

#[derive(Serialize)]
struct PullRequests {
    open: i64,
    closed: i64,
}

mod helpers;

pub fn get_user_data(gh_token: &str, login: String) -> Info {
    let variables = user_info::Variables { login };

    let headers = HeaderMap::from_token(gh_token);
    let client = ClientBuilder::build(headers).expect("Failed to build client");

    let resp = post_graphql_blocking::<UserInfo, _>(&client, API_URL, variables).unwrap();
    let data: user_info::ResponseData = resp.data.unwrap();
    data.user.unwrap().normalize()
}
