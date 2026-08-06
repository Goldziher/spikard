```go
package main

import (
	"encoding/json"
	"testing"
)

func wsConnectHandler(req []byte) ([]byte, error) {
	message := map[string]interface{}{
		"type":    "connection",
		"message": "WebSocket connected",
	}
	return json.Marshal(message)
}

func TestWsConnectHandler(t *testing.T) {
	respBytes, err := wsConnectHandler(nil)
	if err != nil {
		t.Fatalf("handler returned error: %v", err)
	}

	var got map[string]string
	if err := json.Unmarshal(respBytes, &got); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}

	if got["type"] != "connection" {
		t.Errorf("type: got %q want %q", got["type"], "connection")
	}
	if got["message"] != "WebSocket connected" {
		t.Errorf("message: got %q want %q", got["message"], "WebSocket connected")
	}
}
```
