# gRPC Status Codes Reference

Complete reference table for all 17 standard gRPC status codes.

| Code | Numeric | When to Use | HTTP Equivalent |
|------|---------|-------------|-----------------|
| OK | 0 | Request completed successfully | 200 OK |
| CANCELLED | 1 | Operation was cancelled (typically by caller) | 499 Client Closed Request |
| UNKNOWN | 2 | Unknown error or unmapped status from another system | 500 Internal Server Error |
| INVALID_ARGUMENT | 3 | Client specified an invalid argument (validation errors) | 400 Bad Request |
| DEADLINE_EXCEEDED | 4 | Operation deadline was exceeded before completion | 504 Gateway Timeout |
| NOT_FOUND | 5 | Requested entity (e.g., file, user) was not found | 404 Not Found |
| ALREADY_EXISTS | 6 | Entity that client attempted to create already exists | 409 Conflict |
| PERMISSION_DENIED | 7 | Caller lacks permission for the operation | 403 Forbidden |
| RESOURCE_EXHAUSTED | 8 | Resource has been exhausted (quota, rate limit) | 429 Too Many Requests |
| FAILED_PRECONDITION | 9 | Operation rejected because system not in required state | 400 Bad Request |
| ABORTED | 10 | Operation aborted due to concurrency issues | 409 Conflict |
| OUT_OF_RANGE | 11 | Operation attempted past valid range | 400 Bad Request |
| UNIMPLEMENTED | 12 | Operation is not implemented or not supported | 501 Not Implemented |
| INTERNAL | 13 | Internal server error | 500 Internal Server Error |
| UNAVAILABLE | 14 | Service is currently unavailable (temporary condition) | 503 Service Unavailable |
| DATA_LOSS | 15 | Unrecoverable data loss or corruption | 500 Internal Server Error |
| UNAUTHENTICATED | 16 | Request missing or invalid authentication credentials | 401 Unauthorized |

## Usage Guidelines

1. **Choose the most specific code**: Use the most descriptive status code that accurately represents the error condition.

2. **Provide helpful messages**: Include clear, actionable error messages that help clients understand and resolve the issue.

3. **Never expose sensitive information**: Don't include stack traces, database errors, or internal system details in error messages.

4. **Use INTERNAL for unexpected errors**: When encountering unexpected server errors, return INTERNAL and log the details server-side.

5. **Distinguish UNAUTHENTICATED vs PERMISSION_DENIED**: Use UNAUTHENTICATED for missing/invalid credentials, PERMISSION_DENIED for authenticated users lacking permissions.

6. **Consider retry behavior**: Clients may automatically retry certain codes (UNAVAILABLE, DEADLINE_EXCEEDED) but not others (INVALID_ARGUMENT, PERMISSION_DENIED).
