// Ergonomic-layer smoke (Go / struct DTO).
//
// Exercises the ergonomic typed-handler + DTO API end-to-end: a typed handler
// whose body is a Go struct, hydrated by the ergonomic layer, with request
// validation delegated to the Rust core (invalid bodies -> 422 ProblemDetails).
//
// Asserts:
//   - a valid body   -> 2xx with the typed DTO serialized back
//   - an invalid body -> 422 ProblemDetails produced by the Rust CORE (not a
//     Go-side 400), proving validation is delegated to the core.
//
// Exit 0 = pass. Run from this directory with: go run .
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"

	spikard "github.com/Goldziher/spikard"
)

// CreateUser is the request DTO. Non-pointer fields are required; the JSON
// Schema derived from this type drives core-side validation.
type CreateUser struct {
	Name string `json:"name"`
	Age  int    `json:"age"`
}

const baseURL = "http://127.0.0.1:8137"

func post(payload map[string]any) (int, string, error) {
	data, _ := json.Marshal(payload)
	resp, err := http.Post(baseURL+"/users", "application/json", bytes.NewReader(data))
	if err != nil {
		return 0, "", err
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	return resp.StatusCode, string(body), nil
}

func run() int {
	app, err := spikard.NewTypedApp()
	if err != nil {
		fmt.Println("FAIL: NewTypedApp:", err)
		return 1
	}

	err = app.Post("/users", CreateUser{}, func(dto any) (any, error) {
		// The ergonomic layer hands us the validated, hydrated DTO.
		user := dto.(CreateUser)
		fmt.Printf("Handler received: %+v\n", user)
		return user, nil
	})
	if err != nil {
		fmt.Println("FAIL: register route:", err)
		return 1
	}

	// StartBackground blocks until the socket is bound. We intentionally do NOT
	// defer Stop()/Close(): os.Exit tears the process down, and the low-level
	// graceful-shutdown path can block on the running server goroutine.
	if _, err := app.StartBackground("127.0.0.1", 8137); err != nil {
		fmt.Println("FAIL: StartBackground:", err)
		return 1
	}

	// Valid request.
	status, body, err := post(map[string]any{"name": "Alice", "age": 30})
	if err != nil {
		fmt.Println("FAIL: valid POST:", err)
		return 1
	}
	fmt.Printf("VALID   -> %d %s\n", status, body)
	if status < 200 || status >= 300 || !bytes.Contains([]byte(body), []byte("Alice")) {
		fmt.Println("FAIL: valid request did not return the typed DTO")
		return 1
	}

	// Invalid request (age is a string, not an integer).
	status2, body2, err := post(map[string]any{"name": "Bob", "age": "not-a-number"})
	if err != nil {
		fmt.Println("FAIL: invalid POST:", err)
		return 1
	}
	fmt.Printf("INVALID -> %d %s\n", status2, body2)
	if status2 != 422 {
		fmt.Printf("FAIL: invalid body expected 422 from the core, got %d\n", status2)
		return 1
	}

	fmt.Println("ERGO SMOKE PASS (go)")
	return 0
}

func main() {
	os.Exit(run())
}
