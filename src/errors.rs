pub enum Errors {
    /// Authentication Failed - You do not have permissions to access the service
    AuthenticationFailed,
    /// Invalid authentication token supplied
    InvalidAuthenticationTokenSupplied = 4,
    /// This token has not been authorized
    TokenHasNotBeenAuthorized = 14,
    /// This token has expired
    TokenHasExpired = 15,
    /// Invalid service - This service does not exist
    InvalidService = 2,
    /// Invalid Method - No method with that name in this package
    InvalidMethod = 3,
    /// Invalid format - This service doesn't exist in that format
    InvalidFormat = 5,
    /// Invalid parameters - Your request is missing a required parameter
    InvalidParameters = 6,
    /// Invalid resource specified
    InvalidResourceSpecified = 7,
    /// Operation failed - Something else went wrong
    OperationFailed = 8,
    /// Invalid session key - Please re-authenticate
    InvalidSessionKey = 9,
    /// Invalid API key - You must be granted a valid key by last.fm
    InvalidAPIKey = 10,
    /// Service Offline - This service is temporarily offline. Try again later.
    ServiceOffline = 11,
    /// Invalid method signature supplied
    InvalidMethodSignatureSupplied = 13,
    /// There was a temporary error processing your request. Please try again
    TemporaryErrorWhileProcessingRequest = 16,
    /// Suspended API key - Access for your account has been suspended, please contact Last.fm
    SuspendedAPIKey = 26,
    /// Rate limit exceeded - Your IP has made too many requests in a short period
    RatelimitExceeded = 29
}