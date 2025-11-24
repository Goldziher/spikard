```ruby
require "spikard"

app = Spikard::App.new

class ChatHandler < Spikard::WebSocketHandler
  def handle_message(message)
    # Echo JSON message back
    message
  end
end

app.websocket("/chat") { ChatHandler.new }
```
