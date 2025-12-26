#!/usr/bin/env ruby
# frozen_string_literal: true

# Hanami API HTTP server for raw performance benchmarking (NO VALIDATION).
#
# This server implements all workload types to measure Hanami API raw performance
# without validation overhead. It accepts any JSON body and echoes it back.

require 'hanami/api'
require 'json'
require 'date'

# ============================================================================
# Hanami API Application (Raw - No Validation)
# ============================================================================

class BenchmarkApp < Hanami::API
  # JSON body endpoints - parse and echo without validation
  post '/json/small' do
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  post '/json/medium' do
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  post '/json/large' do
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  post '/json/very-large' do
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  # Multipart form endpoints (stub implementations - same as validated version)
  post '/multipart/small' do
    json({ files_received: 1, total_bytes: 1024 })
  end

  post '/multipart/medium' do
    json({ files_received: 2, total_bytes: 10240 })
  end

  post '/multipart/large' do
    json({ files_received: 5, total_bytes: 102400 })
  end

  # URL-encoded form endpoints - accept params without validation
  post '/urlencoded/simple' do
    json(params.select { |k, _| k != 'route' })
  end

  post '/urlencoded/complex' do
    json(params.select { |k, _| k != 'route' })
  end

  # Path parameter endpoints
  get '/path/simple/:id' do
    json({ id: params[:id] })
  end

  get '/path/multiple/:user_id/:post_id' do
    json({
      user_id: params[:user_id],
      post_id: params[:post_id]
    })
  end

  get '/path/deep/:org/:team/:project/:resource/:id' do
    json({
      org: params[:org],
      team: params[:team],
      project: params[:project],
      resource: params[:resource],
      id: params[:id]
    })
  end

  get '/path/int/:id' do
    id_str = params[:id]
    id_int = id_str =~ /\A-?\d+\z/ ? id_str.to_i : id_str.to_i
    json({ id: id_int })
  end

  get '/path/uuid/:uuid' do
    json({ uuid: params[:uuid] })
  end

  get '/path/date/:date' do
    date_str = params[:date]
    begin
      Date.iso8601(date_str)
    rescue ArgumentError
      # In raw mode, we still convert but don't validate strictly
    end
    json({ date: date_str })
  end

  # Query parameter endpoints
  get '/query/few' do
    json({
      q: params[:q],
      page: params[:page]&.to_i,
      limit: params[:limit]&.to_i
    })
  end

  get '/query/medium' do
    json(params.select { |k, _| k != 'route' })
  end

  get '/query/many' do
    json(params.select { |k, _| k != 'route' })
  end

  # Health check endpoints
  get '/health' do
    json({ status: 'ok' })
  end

  get '/' do
    json({ status: 'ok' })
  end
end

# ============================================================================
# Server Startup
# ============================================================================

if __FILE__ == $PROGRAM_NAME
  require 'rackup'

  port = (ARGV[0] || 8000).to_i
  $stderr.puts "[hanami-api-raw] Starting server on port #{port}"

  handler = Rackup::Handler.get('puma')
  handler.run(
    BenchmarkApp.new,
    Port: port,
    Host: '0.0.0.0',
    Threads: '1:1',
    Silent: true
  )
end
