#!/usr/bin/env ruby
# frozen_string_literal: true

# Advanced dependency injection example with factory dependencies.
#
# This example demonstrates:
# - Factory dependencies with blocks
# - Nested dependencies (depends_on)
# - Singleton vs per-request dependencies
# - Simulated database connection pool

require 'bundler/setup'
require 'spikard'

# Simulated database connection pool
class DatabasePool
  attr_reader :connection_string, :pool_size

  def initialize(connection_string, pool_size: 5)
    @connection_string = connection_string
    @pool_size = pool_size
    puts "[DatabasePool] Created with #{pool_size} connections to #{connection_string}"
  end

  def execute(query)
    puts "[DatabasePool] Executing: #{query}"
    {
      success: true,
      query: query,
      timestamp: Time.now.to_i
    }
  end
end

# Simulated cache
class CacheClient
  def initialize(ttl:)
    @ttl = ttl
    @store = {}
    puts "[CacheClient] Initialized with TTL=#{ttl}s"
  end

  def get(key)
    @store[key]
  end

  def set(key, value)
    @store[key] = value
    puts "[CacheClient] Cached: #{key}"
  end
end

app = Spikard::App.new

# Register configuration values
app.provide('database_url', 'postgresql://localhost/myapp')
app.provide('cache_ttl', 300)

# Register database pool as singleton (shared across all requests)
app.provide('db_pool', depends_on: ['database_url'], singleton: true) do |database_url:|
  DatabasePool.new(database_url, pool_size: 10)
end

# Register cache client as singleton
app.provide('cache', depends_on: ['cache_ttl'], singleton: true) do |cache_ttl:|
  CacheClient.new(ttl: cache_ttl)
end

# Register request ID as non-cacheable (new value every time)
app.provide('request_id', cacheable: false) do
  SecureRandom.uuid
end

# Handler that uses database pool
app.get('/users') do |db_pool:, request_id:|
  users = db_pool.execute("SELECT * FROM users")
  {
    request_id: request_id,
    data: users
  }
end

# Handler that uses cache
app.get('/config') do |cache:, database_url:|
  cached_config = cache.get('config')

  if cached_config
    puts '[Handler] Using cached config'
    config = cached_config
  else
    puts '[Handler] Loading fresh config'
    config = {
      database: database_url,
      loaded_at: Time.now.to_i
    }
    cache.set('config', config)
  end

  config
end

# Handler that uses multiple dependencies
app.post('/reports') do |db_pool:, cache:, request_id:|
  # Generate report
  data = db_pool.execute("SELECT COUNT(*) FROM reports WHERE status='pending'")

  # Cache result
  cache.set("report:#{request_id}", data)

  {
    request_id: request_id,
    report: data,
    cached: true
  }
end

puts 'Starting server with advanced dependency injection...'
puts 'Try:'
puts '  curl http://localhost:8000/users'
puts '  curl http://localhost:8000/config'
puts '  curl -X POST http://localhost:8000/reports'
puts ''
puts 'Notice:'
puts '- Database pool is created once (singleton)'
puts '- Cache is shared across requests (singleton)'
puts '- Request IDs are unique per request (non-cacheable)'

app.run(port: 8000)
