# Validation Flows

Validation keeps handlers simple by enforcing contracts at the edge.

## Validate requests

=== "Python"

    ```python
    from msgspec import Struct

    class Payment(Struct):
        id: str
        amount: float

    @app.post("/payments")
    async def create_payment(payment: Payment) -> Payment:
        return payment
    ```

=== "TypeScript"

    ```typescript
    import { z } from "zod";

    const Payment = z.object({
      id: z.string().uuid(),
      amount: z.number().positive(),
    });

    app.post("/payments", ({ body }) => Payment.parse(body));
    ```

=== "Ruby"

    ```ruby
    PaymentSchema = Dry::Schema.Params do
      required(:id).filled(:string)
      required(:amount).filled(:float)
    end

    App.post("/payments") do |ctx|
      PaymentSchema.call(ctx.json)
    end
    ```

=== "Rust"

    ```rust
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(Deserialize, JsonSchema)]
    struct Payment {
        id: String,
        amount: f64,
    }

    app.route(
        post("/payments").request_body::<Payment>().response_body::<Payment>(),
        |ctx: Context| async move {
            let payment: Payment = ctx.json()?;
            Ok(Json(payment))
        },
    )?;
    ```

## Validate responses

Enable response validation on routes that require strict contracts by registering response DTOs/schemas, as in the Rust example above. Keep schemas in version control so generated clients and fixtures stay aligned.

## Testing contracts
- Use the CLI generators to create fixtures/tests from OpenAPI/AsyncAPI.
- Keep schemas in version control; run `task test` to ensure parity across bindings.
- Add ADR updates when changing validation behavior.
