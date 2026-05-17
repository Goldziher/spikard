---
id: ruby_response_basic
language: ruby
title: Response Basic
tags:
  - ruby
---

```ruby
require "spikard"

app = Spikard::App.new

app.get("/health") { |_params, _query, _body| { status: "ok" } }
```
