```go
package main

import (
	"encoding/json"
	"io"
	"net/http"
	"testing"
	"time"

	spikard "github.com/Goldziher/spikard/packages/go"
)

func TestHelloRoute(t *testing.T) {
	app, err := spikard.NewApp()
	if err != nil {
		t.Fatalf("new app: %v", err)
	}
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"message": "Hello, World!"})
	}, "/hello")

	handle, err := app.StartBackground("127.0.0.1", 8081)
	if err != nil {
		t.Fatalf("start background: %v", err)
	}
	defer handle.Stop()

	time.Sleep(100 * time.Millisecond)

	resp, err := http.Get("http://127.0.0.1:8081/hello")
	if err != nil {
		t.Fatalf("request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Fatalf("status: got %d want %d", resp.StatusCode, http.StatusOK)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("read body: %v", err)
	}

	var got map[string]string
	if err := json.Unmarshal(body, &got); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}

	if got["message"] != "Hello, World!" {
		t.Fatalf("message: got %q want %q", got["message"], "Hello, World!")
	}
}
```
