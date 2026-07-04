#!/usr/bin/env ruby
# frozen_string_literal: true

# Drives the ergonomic smoke server with real HTTP requests and asserts:
#
# - a valid body  -> 2xx with the typed DTO serialized back
# - an invalid body -> 422 ProblemDetails produced by the Rust CORE (not a
#   language-side 400), proving validation is delegated to the core.
#
# Exit 0 = pass. Run with the binding-installed Ruby interpreter, e.g.
#     e2e/ergonomic/ruby/check.rb

require "json"
require "net/http"
require "open3"
require "timeout"

HERE = __dir__
PORT = 8000

def post(payload)
  uri = URI("http://127.0.0.1:#{PORT}/users")
  req = Net::HTTP::Post.new(uri)
  req["Content-Type"] = "application/json"
  req.body = JSON.dump(payload)

  http = Net::HTTP.new(uri.host, uri.port)
  http.read_timeout = 5
  resp = http.request(req)
  [resp.code.to_i, resp.body]
rescue StandardError => e
  # Connection failed
  raise e
end

def main
  log_path = File.join(HERE, ".server.log")
  log_file = File.open(log_path, "w")

  # Spawn server in background
  proc = Process.spawn(
    "ruby",
    File.join(HERE, "server.rb"),
    out: log_file,
    err: log_file,
    pgroup: true
  )

  begin
    # Wait for server to bind (with early-exit detection)
    bound = false
    60.times do
      if Process.wait(proc, Process::WNOHANG)
        log_file.flush
        log_content = File.read(log_path)[0, 2500]
        puts("FAIL: server exited early rc=#{$?.exitstatus}")
        puts(log_content)
        return 1
      end

      begin
        post(name: "warmup", age: 1)
        bound = true
        break
      rescue Errno::ECONNREFUSED
        sleep(0.25)
      end
    end

    unless bound
      puts("FAIL: server never came up")
      return 1
    end

    # Test valid request
    status, body = post(name: "Alice", age: 30)
    puts("VALID   -> #{status} #{body}")
    if status < 200 || status >= 300 || !body.include?("Alice")
      puts("FAIL: valid request did not return the typed DTO")
      return 1
    end

    # Test invalid request (age is string, not integer)
    status2, body2 = post(name: "Bob", age: "not-a-number")
    puts("INVALID -> #{status2} #{body2}")
    if status2 != 422
      puts("FAIL: invalid body expected 422 from the core, got #{status2}")
      return 1
    end

    puts("ERGO SMOKE PASS (ruby)")
    0
  ensure
    # Terminate the process group to kill the server and any children.
    begin
      # SIGTERM to the process group
      Process.kill(-15, proc)
      begin
        Timeout.timeout(5) { Process.wait(proc) }
      rescue Timeout::Error
        # SIGKILL if SIGTERM didn't work
        Process.kill(-9, proc)
        Process.wait(proc)
      end

    rescue Errno::ESRCH, Errno::ECHILD
      # Process already gone
    end

    log_file&.close
  end
end

exit(main)
