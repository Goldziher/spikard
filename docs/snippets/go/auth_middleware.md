```go
package main

import (
	"encoding/json"
	"fmt"
	"strings"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type RequestData struct {
	Headers map[string]interface{} `json:"headers"`
}

func checkAuthToken(token string) bool {
	return token == "Bearer sk_test_123456"
}

func authHandler(handler spikard.HandlerFunc) spikard.HandlerFunc {
	return func(req []byte) ([]byte, error) {
		var data RequestData
		if err := json.Unmarshal(req, &data); err != nil {
			return nil, fmt.Errorf("unauthorized")
		}

		var authToken string
		for key, val := range data.Headers {
			if strings.ToLower(key) == "authorization" {
				authToken = val.(string)
				break
			}
		}

		if !checkAuthToken(authToken) {
			return nil, fmt.Errorf("unauthorized")
		}

		return handler(req)
	}
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	baseHandler := func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"message": "Access granted"})
	}

	app.Get(authHandler(baseHandler), "/api/data")

	app.Run()
}
```
