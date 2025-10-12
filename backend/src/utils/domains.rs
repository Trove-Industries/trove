use http::Request;

pub fn extract_subdomain<B>(req :&Request<B>) -> Option<String>{

    let host = req.headers().get("host")?.to_str().ok()?;

    let host = host.split(":").next().unwrap_or(host);

    let parts: Vec<&str> = host.split(".").collect();

    if parts.len() >= 2 {
        return if parts[0] == "www" {
            Some(parts[1].to_lowercase())
        } else {
            Some(parts[0].to_lowercase())
        }
    }

    None
}