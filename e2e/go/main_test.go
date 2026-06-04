package e2e_test

import (
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"testing"
	"fmt"
	"io"
	"net"
	"time"
)

func TestMain(m *testing.M) {
	_, filename, _, _ := runtime.Caller(0)
	dir := filepath.Dir(filename)

	// Change to the configured test-documents directory (if it exists) so that fixture
	// file paths like "pdf/fake_memo.pdf" resolve correctly when running go test
	// from e2e/go/. Repos without document fixtures skip chdir and run from e2e/go/.
	testDocumentsDir := filepath.Join(dir, "..", "..", "test_documents")
	if info, err := os.Stat(testDocumentsDir); err == nil && info.IsDir() {
		if err := os.Chdir(testDocumentsDir); err != nil {
			panic(err)
		}
	}

	// If SUT_URL is already set, a parent process started a shared harness.
	// Use it as-is and do NOT spawn our own.
	if os.Getenv("SUT_URL") != "" {
		os.Exit(m.Run())
	}

	// Spawn the harness executable.
	harnessBin := filepath.Join(dir, "cmd", "harness", "harness")
	cmd := exec.Command(harnessBin)
	cmd.Stderr = os.Stderr
	// Keep pipes open so harness doesn't exit immediately.
	stdin, err := cmd.StdinPipe()
	if err != nil {
		panic(fmt.Sprintf("stdin pipe: %v", err))
	}
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		panic(fmt.Sprintf("stdout pipe: %v", err))
	}
	if err := cmd.Start(); err != nil {
		panic(fmt.Sprintf("start harness: %v", err))
	}

	// Poll TCP port 8012 until harness is ready (15s timeout).
	host := "127.0.0.1"
	port := "8012"
	sutURL := "http://" + host + ":" + port
	deadline := time.Now().Add(15 * time.Second)
	for time.Now().Before(deadline) {
		conn, err := net.DialTimeout("tcp", host+":"+port, 500*time.Millisecond)
		if err == nil {
			conn.Close()
			break
		}
		if cmd.ProcessState != nil {
			// Harness exited early.
			stderr, _ := io.ReadAll(os.Stderr)
			panic(fmt.Sprintf("harness died: %s", stderr))
		}
		time.Sleep(100 * time.Millisecond)
	}

	os.Setenv("SUT_URL", sutURL)
	// Drain stdout so the pipe doesn't block.
	go func() { _, _ = io.Copy(io.Discard, stdout) }()

	code := m.Run()

	// Cleanup: close stdin and wait for harness.
	_ = stdin.Close()
	_ = cmd.Process.Signal(os.Interrupt)
	_ = cmd.Wait()

	os.Exit(code)
}
