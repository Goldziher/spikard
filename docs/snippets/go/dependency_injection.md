```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type Database struct {
	name string
}

func (db *Database) GetUser(id int) map[string]interface{} {
	return map[string]interface{}{
		"id":   id,
		"name": "Alice",
	}
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	db := &Database{name: "production"}

	app.Get(func(req []byte) ([]byte, error) {
		user := db.GetUser(1)
		return json.Marshal(user)
	}, "/users/1")

	app.Run()
}
```
