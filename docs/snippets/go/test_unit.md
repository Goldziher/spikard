```go
package main

import (
	"encoding/json"
	"testing"
)

type CreateUserRequest struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

func createUserHandler(req []byte) ([]byte, error) {
	var user CreateUserRequest
	if err := json.Unmarshal(req, &user); err != nil {
		return nil, err
	}
	return json.Marshal(map[string]interface{}{
		"id":    1,
		"name":  user.Name,
		"email": user.Email,
	})
}

func TestCreateUserHandler(t *testing.T) {
	req, err := json.Marshal(CreateUserRequest{Name: "Alice", Email: "alice@example.com"})
	if err != nil {
		t.Fatalf("marshal request: %v", err)
	}

	respBytes, err := createUserHandler(req)
	if err != nil {
		t.Fatalf("handler returned error: %v", err)
	}

	var got map[string]interface{}
	if err := json.Unmarshal(respBytes, &got); err != nil {
		t.Fatalf("unmarshal response: %v", err)
	}

	if got["name"] != "Alice" {
		t.Errorf("name: got %v want %q", got["name"], "Alice")
	}
	if got["email"] != "alice@example.com" {
		t.Errorf("email: got %v want %q", got["email"], "alice@example.com")
	}
}
```
