```go
package main

import (
	"encoding/json"
	"log"
	"time"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func observabilityMiddleware(handler spikard.HandlerFunc) spikard.HandlerFunc {
	return func(req []byte) ([]byte, error) {
		start := time.Now()
		log.Printf("Request started at %v", start)

		resp, err := handler(req)

		duration := time.Since(start)
		log.Printf("Request completed in %v ms", duration.Milliseconds())

		if err != nil {
			log.Printf("Request error: %v", err)
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

	app.Get(observabilityMiddleware(baseHandler), "/health")

	app.Run()
}
```
