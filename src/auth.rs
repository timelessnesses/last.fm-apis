struct Auth;
use crate::utils::RequestStatus;
use dxr_client;
struct AuthGetSessionResponse {
    status: RequestStatus,
    name: String,
    key: String,
    subscriber: u64
}

impl Auth {
    pub async fn get_session(api_key: impl AsRef<str>, token: impl AsRef<str>, api_signature: impl AsRef<[u8]>) -> AuthGetSessionResponse {
        dxr_client::Call
    }
}