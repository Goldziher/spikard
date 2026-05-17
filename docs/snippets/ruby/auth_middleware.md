---
id: ruby_auth_middleware
language: ruby
title: Auth Middleware
tags:
  - ruby
---

```ruby
require "spikard"

app = Spikard::App.new

app.pre_handler do |request|
  headers = request[:headers] || {}
  if headers["authorization"] != "Bearer dev-token"
    { error: "unauthorized" }
  else
    request
  end
end
```
