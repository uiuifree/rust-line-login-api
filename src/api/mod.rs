use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{LineApiResponse, LineLoginClient};

impl LineLoginClient {
    pub async fn create_token(&self, code: &str, redirect_uri: &str)
                              -> LineApiResponse<LineLoginCreateTokenResponse>
    {
        let req = LineLoginCreateTokenRequest {
            grant_type: "authorization_code".to_string(),
            code: code.to_string(),
            redirect_uri: redirect_uri.to_string(),
            client_id: self.client_id(),
            client_secret: self.client_secret(),
            code_verifier: None,
        };

        self.http_post("https://api.line.me/oauth2/v2.1/token", req, "")
            .await
    }

    pub async fn token_verify(
        &self,
        token: &str,
    ) -> LineApiResponse<LineLoginTokenVerifyResponse> {
        self.http_get(format!("https://api.line.me/oauth2/v2.1/verify?access_token={}", token).as_str(), &json!({}), "")
            .await
    }
    pub async fn update_access_token(
        &self,
        refresh_token: &str,
    ) -> LineApiResponse<LineLoginUpdateAccessTokenResponse> {
        let req = LineLoginUpdateAccessTokenRequest {
            grant_type: "refresh_token".to_string(),
            refresh_token: refresh_token.to_string(),
            client_id: self.client_id(),
            client_secret: self.client_secret(),
        };

        self.http_post("https://api.line.me/oauth2/v2.1/token", json!(req), "")
            .await
    }
    pub async fn revoke_access_token(
        &self,
        access_token: &str,
    ) -> LineApiResponse<LineLoginEmptyResponse> {
        let req = LineLoginRevokeAccessTokenRequest {
            access_token: access_token.to_ascii_lowercase(),
            client_id: self.client_id(),
            client_secret: self.client_secret(),
        };
        self.http_post("https://api.line.me/oauth2/v2.1/token", json!(req), "")
            .await
    }
    pub async fn id_token_verify(
        &self,
        id_token: &str,
        nonce: Option<String>,
        user_id: Option<String>,
    ) -> LineApiResponse<LineLoginIdTokenVerifyResponse> {
        let client_id = self.client_id();
        let req = LineLoginIdTokenVerifyRequest{
            id_token: id_token.to_string(),
            client_id,
            nonce,
            user_id,
        };
        self.http_post("https://api.line.me/oauth2/v2.1/verify", req, "").await
    }
    pub async fn user_info(
        &self,
        access_token: &str,
    ) -> LineApiResponse<LineLoginUserInfoResponse> {
        self.http_get("https://api.line.me/oauth2/v2.1/userinfo", &json!({}), access_token)
            .await
    }
    pub async fn profile(
        &self,
        access_token: &str,
    ) -> LineApiResponse<LineLoginProfileResponse> {
        self.http_get("https://api.line.me/v2/profile", &json!({}), access_token)
            .await
    }
    pub async fn friend_ship(
        &self,
        access_token: &str,
    ) -> LineApiResponse<LineLoginFriendShip> {
        self.http_get("https://api.line.me/friendship/v1/status", &json!({}), access_token)
            .await
    }
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginEmptyResponse {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginCreateTokenRequest {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_verifier: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginCreateTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub id_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginTokenVerifyResponse {
    pub scope: String,
    pub client_id: String,
    pub expires_in: u64,
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
struct LineLoginUpdateAccessTokenRequest {
    pub grant_type: String,
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginUpdateAccessTokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub scope: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
struct LineLoginRevokeAccessTokenRequest {
    pub access_token: String,
    pub client_id: String,
    pub client_secret: String,
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginIdTokenVerifyRequest {
    pub id_token: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginIdTokenVerifyResponse {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amr: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginUserInfoResponse {
    pub sub: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginProfileResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pictureUrl")]
    pub picture_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "statusMessage")]
    pub status_message: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineLoginFriendShip {
    #[serde(rename = "friendFlag")]
    pub friend_flag: bool,
}

