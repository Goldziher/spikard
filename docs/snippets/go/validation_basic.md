```go
package main

import (
	"encoding/json"
	"fmt"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type User struct {
	Name string `json:"name"`
	Age  int    `json:"age"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Post(func(req []byte) ([]byte, error) {
		var user User
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
	}, "/users")

	app.Run()
}
```
