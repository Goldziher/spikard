```go
package main

import (
	"encoding/json"
	"fmt"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type CreateUserRequest struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Post(func(req []byte) ([]byte, error) {
		var request CreateUserRequest
		if err := json.Unmarshal(req, &request); err != nil {
			return nil, fmt.Errorf("invalid request body: %w", err)
		}

		if request.Name == "" {
			return nil, fmt.Errorf("name field is required")
		}
		if request.Email == "" {
			return nil, fmt.Errorf("email field is required")
		}

		return json.Marshal(request)
	}, "/users")

	app.Run()
}
```
