```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		message := map[string]interface{}{
			"type":    "connection",
			"message": "WebSocket connected",
		}
		return json.Marshal(message)
	}, "/ws")

	app.Run()
}
```
