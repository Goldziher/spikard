```go
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"sync"
	"testing"
	"time"

	spikard "github.com/Goldziher/spikard/packages/go"
)

func TestUserWorkflow(t *testing.T) {
	app, err := spikard.NewApp()
	if err != nil {
		t.Fatalf("new app: %v", err)
	}
	defer app.Close()

	var mu sync.Mutex
	users := map[int]map[string]interface{}{}
	nextID := 1

	app.Post(func(req []byte) ([]byte, error) {
		var body map[string]interface{}
		if err := json.Unmarshal(req, &body); err != nil {
			return nil, err
		}

		mu.Lock()
		id := nextID
		nextID++
		user := map[string]interface{}{"id": id, "name": body["name"]}
		users[id] = user
		mu.Unlock()

		return json.Marshal(user)
	}, "/users")

	app.Get(func(req []byte) ([]byte, error) {
		var params map[string]interface{}
		if err := json.Unmarshal(req, &params); err != nil {
			return nil, err
		}

		mu.Lock()
		defer mu.Unlock()
		return json.Marshal(users)
	}, "/users")

	handle, err := app.StartBackground("127.0.0.1", 8082)
	if err != nil {
		t.Fatalf("start background: %v", err)
	}
	defer handle.Stop()

	time.Sleep(100 * time.Millisecond)
	baseURL := "http://127.0.0.1:8082"

	createBody, _ := json.Marshal(map[string]string{"name": "Alice"})
	createResp, err := http.Post(baseURL+"/users", "application/json", bytes.NewReader(createBody))
	if err != nil {
		t.Fatalf("create request failed: %v", err)
	}
	defer createResp.Body.Close()

	if createResp.StatusCode != http.StatusOK {
		t.Fatalf("create status: got %d want %d", createResp.StatusCode, http.StatusOK)
	}

	createdBytes, err := io.ReadAll(createResp.Body)
	if err != nil {
		t.Fatalf("read create body: %v", err)
	}

	var created map[string]interface{}
	if err := json.Unmarshal(createdBytes, &created); err != nil {
		t.Fatalf("unmarshal created: %v", err)
	}

	if created["name"] != "Alice" {
		t.Fatalf("created name: got %v want %q", created["name"], "Alice")
	}

	getResp, err := http.Get(fmt.Sprintf("%s/users", baseURL))
	if err != nil {
		t.Fatalf("get request failed: %v", err)
	}
	defer getResp.Body.Close()

	if getResp.StatusCode != http.StatusOK {
		t.Fatalf("get status: got %d want %d", getResp.StatusCode, http.StatusOK)
	}
}
```
