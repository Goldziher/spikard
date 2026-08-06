```go
package main

import (
	"fmt"
	"testing"
)

type UserCreate struct {
	Email    string `json:"email"`
	Age      int    `json:"age"`
	Username string `json:"username"`
}

func validateUser(user UserCreate) error {
	if user.Email == "" || !isValidEmail(user.Email) {
		return fmt.Errorf("email: must be a valid email address")
	}
	if user.Age < 18 {
		return fmt.Errorf("age: must be at least 18")
	}
	if user.Username == "" {
		return fmt.Errorf("username: is required")
	}
	return nil
}

func isValidEmail(email string) bool {
	at := -1
	for i, c := range email {
		if c == '@' {
			at = i
			break
		}
	}
	return at > 0 && at < len(email)-1
}

func TestValidateUserTableDriven(t *testing.T) {
	cases := []struct {
		name    string
		user    UserCreate
		wantErr bool
	}{
		{
			name:    "valid payload",
			user:    UserCreate{Email: "test@example.com", Age: 25, Username: "testuser"},
			wantErr: false,
		},
		{
			name:    "invalid email",
			user:    UserCreate{Email: "not-an-email", Age: 25, Username: "testuser"},
			wantErr: true,
		},
		{
			name:    "age below minimum",
			user:    UserCreate{Email: "test@example.com", Age: 16, Username: "testuser"},
			wantErr: true,
		},
		{
			name:    "missing username",
			user:    UserCreate{Email: "test@example.com", Age: 25},
			wantErr: true,
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			err := validateUser(tc.user)
			if (err != nil) != tc.wantErr {
				t.Errorf("validateUser(%+v) error = %v, wantErr %v", tc.user, err, tc.wantErr)
			}
		})
	}
}
```
