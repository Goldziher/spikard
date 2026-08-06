```go
package validation_test

import (
	"encoding/json"
	"fmt"
	"testing"

	spikard "github.com/Goldziher/spikard/packages/go"
)

type User struct {
	Email    string `json:"email"`
	Age      int    `json:"age"`
	Username string `json:"username"`
}

func validateUser(request []byte) ([]byte, error) {
	var user User
	if err := json.Unmarshal(request, &user); err != nil {
		return nil, fmt.Errorf("invalid request: %w", err)
	}
	if user.Email == "" || user.Username == "" {
		return nil, fmt.Errorf("email and username are required")
	}
	if user.Age < 18 {
		return nil, fmt.Errorf("age must be at least 18")
	}
	return json.Marshal(user)
}

func TestUserCreationValidation(t *testing.T) {
	var handler spikard.HandlerFunc = validateUser

	valid, err := handler([]byte(`{"email":"test@example.com","age":25,"username":"testuser"}`))
	if err != nil {
		t.Fatalf("valid request failed: %v", err)
	}
	if len(valid) == 0 {
		t.Fatal("valid request returned an empty response")
	}

	tests := []struct {
		name string
		body string
	}{
		{name: "invalid email", body: `{"email":"","age":25,"username":"testuser"}`},
		{name: "age below minimum", body: `{"email":"test@example.com","age":16,"username":"testuser"}`},
		{name: "missing username", body: `{"email":"test@example.com","age":25}`},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if _, err := handler([]byte(tt.body)); err == nil {
				t.Fatal("expected validation to reject the request")
			}
		})
	}
}
```
