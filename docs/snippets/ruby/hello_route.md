---
id: ruby_hello_route
language: ruby
title: Hello Route
tags:
  - ruby
---

```ruby
require "spikard"

app = Spikard::App.new

app.get("/users/:id") do |params, _query, _body|
  { id: params[:id].to_i, name: "Alice" }
end

app.run(config: { port: 8000 })
```
