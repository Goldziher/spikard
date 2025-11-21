#!/usr/bin/env ruby
# frozen_string_literal: true

# Spikard Ruby HTTP server for workload benchmarking.
#
# This server implements all workload types to measure Ruby binding performance
# against the pure Rust baseline.

$LOAD_PATH.unshift File.expand_path('../../../../packages/ruby/lib', __dir__)
require 'spikard_rb'
require 'spikard'

app = Spikard::App.new

# ============================================================================
# JSON Body Workloads
# ============================================================================

app.post '/json/small' do |body|
  body
end

app.post '/json/medium' do |body|
  body
end

app.post '/json/large' do |body|
  body
end

app.post '/json/very-large' do |body|
  body
end

# ============================================================================
# Multipart Form Workloads
# ============================================================================

app.post '/multipart/small' do |body|
  { files_received: 1, total_bytes: 1024 }
end

app.post '/multipart/medium' do |body|
  { files_received: 2, total_bytes: 10240 }
end

app.post '/multipart/large' do |body|
  { files_received: 5, total_bytes: 102400 }
end

# ============================================================================
# URL Encoded Form Workloads
# ============================================================================

app.post '/urlencoded/simple' do |body|
  body
end

app.post '/urlencoded/complex' do |body|
  body
end

# ============================================================================
# Path Parameter Workloads
# ============================================================================

app.get '/path/simple/:id' do |params|
  { id: params[:id] }
end

app.get '/path/multiple/:user_id/:post_id' do |params|
  { user_id: params[:user_id], post_id: params[:post_id] }
end

app.get '/path/deep/:org/:team/:project/:api/:item' do |params|
  {
    org: params[:org],
    team: params[:team],
    project: params[:project],
    api: params[:api],
    item: params[:item]
  }
end

app.get '/path/int/:id' do |params|
  { id: params[:id].to_i }
end

app.get '/path/uuid/:id' do |params|
  { id: params[:id] }
end

app.get '/path/date/:date' do |params|
  { date: params[:date] }
end

# ============================================================================
# Query Parameter Workloads
# ============================================================================

app.get '/query/few' do |query|
  query
end

app.get '/query/medium' do |query|
  query
end

app.get '/query/many' do |query|
  query
end

# ============================================================================
# Health Check
# ============================================================================

app.get '/health' do
  { status: 'ok' }
end

app.get '/' do
  { status: 'ok' }
end

# Start server
port = ARGV[0]&.to_i || 8000
app.listen(port, '0.0.0.0')
warn "[spikard-ruby-workloads] Server listening on port #{port}"
