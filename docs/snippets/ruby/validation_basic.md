```ruby
require "spikard"

PaymentSchema = Dry::Schema.Params do
  required(:id).filled(:string)
  required(:amount).filled(:float)
end

app = Spikard::App.new

app.post("/payments") do |_params, _query, body|
  PaymentSchema.call(body)
end
```
