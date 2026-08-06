```go
package main

import (
	"encoding/json"
	"fmt"
	"strings"
	"testing"
)

func checkAuthToken(token string) bool {
	return token == "Bearer sk_test_123456"
}

func authGuard(req []byte) (map[string]interface{}, error) {
	var data struct {
		Headers map[string]interface{} `json:"headers"`
	}
	if err := json.Unmarshal(req, &data); err != nil {
		return nil, fmt.Errorf("unauthorized: %w", err)
	}

	var authToken string
	for key, val := range data.Headers {
		if strings.ToLower(key) == "authorization" {
			authToken, _ = val.(string)
			break
		}
	}

	if !checkAuthToken(authToken) {
		return nil, fmt.Errorf("unauthorized")
	}

	return map[string]interface{}{"user_id": "user-1"}, nil
}

func TestAuthGuardValidToken(t *testing.T) {
	req, _ := json.Marshal(map[string]interface{}{
		"headers": map[string]interface{}{"authorization": "Bearer sk_test_123456"},
	})

	context, err := authGuard(req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if context["user_id"] != "user-1" {
		t.Errorf("user_id: got %v want %q", context["user_id"], "user-1")
	}
}

func TestAuthGuardMissingToken(t *testing.T) {
	req, _ := json.Marshal(map[string]interface{}{
		"headers": map[string]interface{}{},
	})

	_, err := authGuard(req)
	if err == nil {
		t.Fatal("expected an unauthorized error")
	}
}
```
