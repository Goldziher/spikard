# Validation Flows

Validation keeps handlers simple by enforcing contracts at the edge.

## Request Validation
- Define DTOs or schemas per binding (msgspec/Pydantic, Zod, RBS, serde) and register them with the route.
- Coerce and validate path/query/header/cookie/body values before handler code runs.
- Return clear error payloads with pointer paths when validation fails.

## Response Validation
- Enable response validation for critical endpoints to guarantee contract adherence.
- Use shared DTOs so response schemas stay in sync with request types and generated clients.

## Example (Python)
```python
@app.post("/payments")
async def create_payment(payment: Payment) -> Payment:
    return payment
```
The runtime validates `payment` on ingress and the returned value on egress when validation is enabled.

## Example (TypeScript)
```typescript
const Payment = z.object({ id: z.string().uuid(), amount: z.number().positive() });

app.post("/payments", ({ body }) => Payment.parse(body));
```

## Testing Contracts
- Use the CLI generators to create fixtures/tests from OpenAPI/AsyncAPI.
- Keep schemas in version control; use `task test` to ensure parity across bindings.

See [Validation Engine](../concepts/validation.md) for background and ADR notes.
