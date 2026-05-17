---
id: ruby_middleware_basic
language: ruby
title: Middleware Basic
tags:
  - ruby
---

```ruby
require "spikard"

app = Spikard::App.new

app.on_request do |request|
  puts "#{request[:method]} #{request[:path]}"
  request
end
```
