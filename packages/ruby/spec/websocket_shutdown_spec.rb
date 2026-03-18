# frozen_string_literal: true

require 'spec_helper'
require 'rbconfig'

RSpec.describe 'WebSocket shutdown' do
  it 'lets a process with websocket handlers exit cleanly' do
    script = <<~RUBY
      require_relative 'lib/spikard'

      handler_class = Class.new(Spikard::WebSocketHandler) do
        def handle_message(message)
          message
        end
      end

      app = Spikard::App.new
      app.websocket('/chat') { handler_class.new }

      client = Spikard::TestClient.new(app)
      ws = client.websocket('/chat')
      ws.send_json({ 'message' => 'hello' })
      ws.receive_json
      ws.close
      client.close

      puts 'clean-exit'
    RUBY

    stdout, stderr, status = run_subprocess(script, timeout_seconds: 15)

    expect(status).not_to be_nil
    expect(status.success?).to be(true), "stdout: #{stdout}\nstderr: #{stderr}"
    expect(stdout).to include('clean-exit')
  end

  def run_subprocess(script, timeout_seconds:)
    stdout_r, stdout_w = IO.pipe
    stderr_r, stderr_w = IO.pipe
    pid = spawn_subprocess(script, stdout_w, stderr_w)
    close_writers(stdout_w, stderr_w)
    status = wait_for_subprocess(pid, timeout_seconds)
    [stdout_r.read, stderr_r.read, status]
  ensure
    close_writers(stdout_w, stderr_w)
    close_readers(stdout_r, stderr_r)
    reap_subprocess(pid)
  end

  def subprocess_env
    {
      'BUNDLE_GEMFILE' => ENV.fetch('BUNDLE_GEMFILE', nil),
      'BUNDLE_PATH' => ENV.fetch('BUNDLE_PATH', nil),
      'BUNDLE_BIN_PATH' => ENV.fetch('BUNDLE_BIN_PATH', nil),
      'GEM_HOME' => ENV.fetch('GEM_HOME', nil),
      'GEM_PATH' => ENV.fetch('GEM_PATH', nil),
      'RUBYOPT' => ENV.fetch('RUBYOPT', nil)
    }.compact
  end

  def spawn_subprocess(script, stdout_w, stderr_w)
    Process.spawn(
      subprocess_env,
      RbConfig.ruby,
      '-I',
      'lib',
      '-e',
      script,
      chdir: File.expand_path('..', __dir__),
      out: stdout_w,
      err: stderr_w
    )
  end

  def wait_for_subprocess(pid, timeout_seconds)
    deadline = Process.clock_gettime(Process::CLOCK_MONOTONIC) + timeout_seconds

    loop do
      waited_pid, waited_status = Process.waitpid2(pid, Process::WNOHANG)
      return waited_status if waited_pid

      handle_subprocess_timeout(pid, timeout_seconds) if deadline_reached?(deadline)
      sleep 0.05
    end
  end

  def deadline_reached?(deadline)
    Process.clock_gettime(Process::CLOCK_MONOTONIC) >= deadline
  end

  def handle_subprocess_timeout(pid, timeout_seconds)
    terminate_subprocess(pid)
    raise "subprocess did not exit within #{timeout_seconds} seconds"
  end

  def terminate_subprocess(pid)
    Process.kill('TERM', pid)
    sleep 0.1
    Process.kill('KILL', pid)
  rescue Errno::ESRCH
    nil
  end

  def close_writers(*writers)
    writers.each do |writer|
      writer.close unless writer.closed?
    end
  end

  def close_readers(*readers)
    readers.each do |reader|
      reader.close unless reader.closed?
    end
  end

  def reap_subprocess(pid)
    return unless pid

    Process.wait(pid)
  rescue Errno::ECHILD
    nil
  end
end
