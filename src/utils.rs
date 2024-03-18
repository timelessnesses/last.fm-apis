use md5;

pub fn api_signature(api_key: impl AsRef<str>, method: impl AsRef<str>, token: impl AsRef<str>, secret: impl AsRef<str>) -> String {
    let connected = format!("api_key{}method{}token{}{}", api_key.as_ref(), method.as_ref(), token.as_ref(), secret.as_ref());
    return hex::encode(md5::compute(connected).0)
}

pub enum RequestStatus {
    Ok,
    Failed
}