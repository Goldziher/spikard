```go
package main

import (
	"encoding/json"
	"fmt"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type HeaderRequest struct {
	Headers map[string]interface{} `json:"headers"`
}

func authMiddleware(handler spikard.HandlerFunc) spikard.HandlerFunc {
	return func(req []byte) ([]byte, error) {
		var headerReq HeaderRequest
		if err := json.Unmarshal(req, &headerReq); err != nil {
			return nil, fmt.Errorf("unauthorized")
		}

		authHeader, ok := headerReq.Headers["authorization"].(string)
		if !ok || authHeader == "" {
			return nil, fmt.Errorf("unauthorized")
		}

		if authHeader != "Bearer valid-token" {
			return nil, fmt.Errorf("unauthorized")
		}

		return handler(req)
	}
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	baseHandler := func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"status": "authenticated"})
	}

	app.Get(authMiddleware(baseHandler), "/protected")

	app.Run()
}
```
