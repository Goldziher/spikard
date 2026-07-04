```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type User struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"status": "ok"})
	}, "/health")

	app.Get(func(req []byte) ([]byte, error) {
		users := []User{{ID: 1, Name: "Alice"}, {ID: 2, Name: "Bob"}}
		return json.Marshal(users)
	}, "/users")

	app.Post(func(req []byte) ([]byte, error) {
		var user User
		if err := json.Unmarshal(req, &user); err != nil {
			return nil, err
		}
		return json.Marshal(user)
	}, "/users")

	app.Delete(func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"status": "deleted"})
	}, "/users/{id}")

	app.Run()
}
```
