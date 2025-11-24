```ruby
App.use do |ctx, next_middleware|
  puts "#{ctx.method} #{ctx.path}"
  next_middleware.call
end
```
