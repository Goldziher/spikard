---
id: ruby_validation_error_format
language: ruby
title: Validation Error Format
tags:
  - ruby
---

```ruby
def format_validation_errors(result)
  {
    error: "validation_failed",
    message: "Request validation failed",
    details: result.errors.messages.map do |msg|
      {
        field: msg.path.join("."),
        message: msg.text,
        type: msg.predicate.to_s
      }
    end
  }
end

app.post("/users") do |_params, _query, body|
  result = CreateUserSchema.call(body)

  if result.failure?
    halt 422, format_validation_errors(result)
  end

  # Process valid request
  { id: "usr_123", email: result[:email] }
end
```
