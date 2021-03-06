//! [POST /_matrix/identity/v2/validate/msisdn/submitToken](https://matrix.org/docs/spec/identity_service/r0.3.0#post-matrix-identity-v2-validate-msisdn-submittoken)

use ruma_api::ruma_api;

ruma_api! {
    metadata: {
        description: "Validate ownership of an phone number.",
        method: POST,
        name: "validate_msisdn",
        path: "/_matrix/identity/v2/validate/msisdn/submitToken",
        authentication: AccessToken,
        rate_limited: false,
    }

    request: {
        /// The session ID, generated by the `requestToken` call.
        pub sid: &'a str,

        /// The client secret that was supplied to the `requestToken` call.
        pub client_secret: &'a str,

        /// The token generated by the `requestToken` call and sent to the user.
        pub token: &'a str,
    }

    response: {
        /// Whether the validation was successful or not.
        pub success: bool,
    }
}

impl<'a> Request<'a> {
    /// Create a new `Request` with the given session ID, client secret and token.
    pub fn new(sid: &'a str, client_secret: &'a str, token: &'a str) -> Self {
        Self { sid, client_secret, token }
    }
}

impl Response {
    /// Create a new `Response` with the success status.
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
