```ruby
require "spikard"

app = Spikard::App.new

# Value dependency (singleton)
app.provide("config", { "db_url" => "postgresql://localhost/app" })

# Factory dependency (depends on config, singleton)
app.provide("db_pool", depends_on: ["config"], singleton: true) do |config:|
  { url: config["db_url"], client: "pool" }
end

app.get("/stats") do |_params, _query, _body, config:, db_pool:|
  { db: db_pool[:url], env: config["db_url"] }
end
```
