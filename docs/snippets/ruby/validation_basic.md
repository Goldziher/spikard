```ruby
PaymentSchema = Dry::Schema.Params do
  required(:id).filled(:string)
  required(:amount).filled(:float)
end

App.post("/payments") do |ctx|
  PaymentSchema.call(ctx.json)
end
```
