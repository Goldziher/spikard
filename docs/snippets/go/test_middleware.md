```go
package main

import (
	"encoding/json"
	"fmt"
	"testing"

	spikard "github.com/Goldziher/spikard/packages/go"
)

func requireAuth(handler spikard.HandlerFunc) spikard.HandlerFunc {
	return func(req []byte) ([]byte, error) {
		var data struct {
			Headers map[string]interface{} `json:"headers"`
		}
		if err := json.Unmarshal(req, &data); err != nil {
			return nil, fmt.Errorf("unauthorized")
		}

		token, _ := data.Headers["authorization"].(string)
		if token == "" {
			return nil, fmt.Errorf("unauthorized")
		}

		return handler(req)
	}
}

func TestRequireAuthRejectsMissingToken(t *testing.T) {
	protected := requireAuth(func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"data": "secret"})
	})

	req, _ := json.Marshal(map[string]interface{}{"headers": map[string]interface{}{}})

	_, err := protected(req)
	if err == nil {
		t.Fatal("expected an error for missing authorization header")
	}
}

func TestRequireAuthAllowsValidToken(t *testing.T) {
	protected := requireAuth(func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"data": "secret"})
	})

	req, _ := json.Marshal(map[string]interface{}{
		"headers": map[string]interface{}{"authorization": "Bearer token123"},
	})

	respBytes, err := protected(req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var resp map[string]string
	if err := json.Unmarshal(respBytes, &resp); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if resp["data"] != "secret" {
		t.Errorf("data: got %q want %q", resp["data"], "secret")
	}
}
```
