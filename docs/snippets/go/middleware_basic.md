```go
package main

import (
	"encoding/json"
	"log"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func loggingMiddleware(handler spikard.HandlerFunc) spikard.HandlerFunc {
	return func(req []byte) ([]byte, error) {
		log.Printf("Request received: %s", string(req))
		resp, err := handler(req)
		if err != nil {
			log.Printf("Error: %v", err)
		}
		return resp, err
	}
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	baseHandler := func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"status": "ok"})
	}

	wrappedHandler := loggingMiddleware(baseHandler)

	app.Get(wrappedHandler, "/health")

	app.Run()
}
```
