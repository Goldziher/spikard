```go
package main

import (
	"encoding/json"
	"fmt"
	"testing"
)

type UserCreate struct {
	Name string `json:"name"`
	Age  int    `json:"age"`
}

func createUserHandler(req []byte) ([]byte, error) {
	var user UserCreate
	if err := json.Unmarshal(req, &user); err != nil {
		return nil, fmt.Errorf("invalid request: %w", err)
	}
	if user.Name == "" {
		return nil, fmt.Errorf("name is required")
	}
	if user.Age < 0 {
		return nil, fmt.Errorf("age must be positive")
	}
	return json.Marshal(user)
}

func TestCreateUserRejectsInvalidAgeType(t *testing.T) {
	req := []byte(`{"name": "Bob", "age": "not a number"}`)

	_, err := createUserHandler(req)
	if err == nil {
		t.Fatal("expected a validation error for a non-numeric age")
	}
}

func TestCreateUserRejectsNegativeAge(t *testing.T) {
	req, _ := json.Marshal(UserCreate{Name: "Bob", Age: -1})

	_, err := createUserHandler(req)
	if err == nil {
		t.Fatal("expected a validation error for a negative age")
	}
}

func TestCreateUserAcceptsValidPayload(t *testing.T) {
	req, _ := json.Marshal(UserCreate{Name: "Bob", Age: 30})

	respBytes, err := createUserHandler(req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var got UserCreate
	if err := json.Unmarshal(respBytes, &got); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if got.Name != "Bob" || got.Age != 30 {
		t.Errorf("got %+v want %+v", got, UserCreate{Name: "Bob", Age: 30})
	}
}
```
