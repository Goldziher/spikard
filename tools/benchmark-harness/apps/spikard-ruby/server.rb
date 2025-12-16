#!/usr/bin/env ruby
# frozen_string_literal: true

# Spikard Ruby HTTP server for workload benchmarking.
#
# This server implements all workload types to measure Ruby binding performance.

$LOAD_PATH.unshift File.expand_path('../../../../packages/ruby/lib', __dir__)
require 'spikard_rb'
require 'spikard'
require 'json'

GC::Profiler.enable

def write_benchmark_metrics
  metrics = {
    gc_count: GC.count,
    gc_time_ms: (GC::Profiler.total_time.to_f * 1000.0),
    heap_allocated_pages: GC.stat[:heap_allocated_pages],
    heap_live_slots: GC.stat[:heap_live_slots]
  }

  path = ENV.fetch('SPIKARD_RUBY_METRICS_FILE', "/tmp/ruby-metrics-#{Process.pid}.json")
  File.write(path, JSON.pretty_generate(metrics))
rescue StandardError
  nil
end

Signal.trap('TERM') { write_benchmark_metrics; exit(0) }
Signal.trap('INT') { write_benchmark_metrics; exit(0) }
at_exit { write_benchmark_metrics }

app = Spikard::App.new

# ============================================================================
# Validation Classes
# ============================================================================

class SmallPayload
  attr_accessor :name, :description, :price, :tax

  def initialize(data)
    @name = data['name'] || data[:name]
    @description = data['description'] || data[:description]
    @price = data['price'] || data[:price]
    @tax = data['tax'] || data[:tax]

    raise ArgumentError, "name must be a String" unless @name.is_a?(String)
    raise ArgumentError, "description must be a String" unless @description.is_a?(String)
    raise ArgumentError, "price must be a Numeric" unless @price.is_a?(Numeric)
    raise ArgumentError, "tax must be a Numeric or nil" unless @tax.nil? || @tax.is_a?(Numeric)
  end

  def to_h
    { name: @name, description: @description, price: @price, tax: @tax }
  end
end

class Address
  attr_accessor :street, :city, :state, :zip_code

  def initialize(data)
    @street = data['street'] || data[:street]
    @city = data['city'] || data[:city]
    @state = data['state'] || data[:state]
    @zip_code = data['zip_code'] || data[:zip_code]

    raise ArgumentError, "street must be a String" unless @street.is_a?(String)
    raise ArgumentError, "city must be a String" unless @city.is_a?(String)
    raise ArgumentError, "state must be a String" unless @state.is_a?(String)
    raise ArgumentError, "zip_code must be a String" unless @zip_code.is_a?(String)
  end

  def to_h
    { street: @street, city: @city, state: @state, zip_code: @zip_code }
  end
end

class MediumPayload
  attr_accessor :user_id, :username, :email, :is_active, :address, :tags

  def initialize(data)
    @user_id = data['user_id'] || data[:user_id]
    @username = data['username'] || data[:username]
    @email = data['email'] || data[:email]
    @is_active = data['is_active'] || data[:is_active]
    @address = Address.new(data['address'] || data[:address])
    @tags = data['tags'] || data[:tags]

    raise ArgumentError, "user_id must be an Integer" unless @user_id.is_a?(Integer)
    raise ArgumentError, "username must be a String" unless @username.is_a?(String)
    raise ArgumentError, "email must be a String" unless @email.is_a?(String)
    raise ArgumentError, "tags must be an Array" unless @tags.is_a?(Array)
  end

  def to_h
    { user_id: @user_id, username: @username, email: @email, is_active: @is_active, address: @address.to_h, tags: @tags }
  end
end

class Item
  attr_accessor :id, :name, :price, :in_stock

  def initialize(data)
    @id = data['id'] || data[:id]
    @name = data['name'] || data[:name]
    @price = data['price'] || data[:price]
    @in_stock = data['in_stock'] || data[:in_stock]

    raise ArgumentError, "id must be an Integer" unless @id.is_a?(Integer)
    raise ArgumentError, "name must be a String" unless @name.is_a?(String)
    raise ArgumentError, "price must be a Numeric" unless @price.is_a?(Numeric)
  end

  def to_h
    { id: @id, name: @name, price: @price, in_stock: @in_stock }
  end
end

class LargePayload
  attr_accessor :order_id, :customer_name, :items, :total, :notes

  def initialize(data)
    @order_id = data['order_id'] || data[:order_id]
    @customer_name = data['customer_name'] || data[:customer_name]
    @items = (data['items'] || data[:items]).map { |item| Item.new(item) }
    @total = data['total'] || data[:total]
    @notes = data['notes'] || data[:notes]

    raise ArgumentError, "order_id must be a String" unless @order_id.is_a?(String)
    raise ArgumentError, "customer_name must be a String" unless @customer_name.is_a?(String)
    raise ArgumentError, "items must be an Array" unless @items.is_a?(Array)
    raise ArgumentError, "total must be a Numeric" unless @total.is_a?(Numeric)
    raise ArgumentError, "notes must be a String" unless @notes.is_a?(String)
  end

  def to_h
    { order_id: @order_id, customer_name: @customer_name, items: @items.map(&:to_h), total: @total, notes: @notes }
  end
end

# ============================================================================
# JSON Body Workloads
# ============================================================================

app.post '/json/small', handler_name: 'post_json_small' do |request = {}|
  body = request.is_a?(Hash) ? (request[:body] || request['body'] || {}) : {}
  body
end

app.post '/json/medium', handler_name: 'post_json_medium' do |request = {}|
  body = request.is_a?(Hash) ? (request[:body] || request['body'] || {}) : {}
  body
end

app.post '/json/large', handler_name: 'post_json_large' do |request = {}|
  body = request.is_a?(Hash) ? (request[:body] || request['body'] || {}) : {}
  body
end

app.post '/json/very-large', handler_name: 'post_json_very_large' do |request = {}|
  body = request.is_a?(Hash) ? (request[:body] || request['body'] || {}) : {}
  body
end

# ============================================================================
# Multipart Form Workloads
# ============================================================================

app.post '/multipart/small', handler_name: 'post_multipart_small' do |request|
  { files_received: 1, total_bytes: 1024 }
end

app.post '/multipart/medium', handler_name: 'post_multipart_medium' do |request|
  { files_received: 2, total_bytes: 10240 }
end

app.post '/multipart/large', handler_name: 'post_multipart_large' do |request|
  { files_received: 5, total_bytes: 102400 }
end

# ============================================================================
# URL Encoded Form Workloads
# ============================================================================

app.post '/urlencoded/simple', handler_name: 'post_urlencoded_simple' do |request|
  request.is_a?(Hash) ? (request[:body] || request['body'] || {}) : {}
end

app.post '/urlencoded/complex', handler_name: 'post_urlencoded_complex' do |request|
  request.is_a?(Hash) ? (request[:body] || request['body'] || {}) : {}
end

# ============================================================================
# Path Parameter Workloads
# ============================================================================

app.get '/path/simple/{id}', handler_name: 'get_path_simple' do |request|
  { id: request[:path_params][:id] }
end

app.get '/path/multiple/{user_id}/{post_id}', handler_name: 'get_path_multiple' do |request|
  {
    user_id: request[:path_params][:user_id],
    post_id: request[:path_params][:post_id]
  }
end

app.get '/path/deep/{org}/{team}/{project}/{resource}/{id}', handler_name: 'get_path_deep' do |request|
  {
    org: request[:path_params][:org],
    team: request[:path_params][:team],
    project: request[:path_params][:project],
    resource: request[:path_params][:resource],
    id: request[:path_params][:id]
  }
end

app.get '/path/int/{id}', handler_name: 'get_path_int' do |request|
  { id: request[:path_params][:id].to_i }
end

app.get '/path/uuid/{uuid}', handler_name: 'get_path_uuid' do |request|
  { uuid: request[:path_params][:uuid] }
end

app.get '/path/date/{date}', handler_name: 'get_path_date' do |request|
  { date: request[:path_params][:date] }
end

# ============================================================================
# Query Parameter Workloads
# ============================================================================

app.get '/query/few', handler_name: 'get_query_few' do |request|
  request[:query_params] || {}
end

app.get '/query/medium', handler_name: 'get_query_medium' do |request|
  request[:query_params] || {}
end

app.get '/query/many', handler_name: 'get_query_many' do |request|
  request[:query_params] || {}
end

# ============================================================================
# Health Check
# ============================================================================

app.get '/health', handler_name: 'get_health' do |request|
  { status: 'ok' }
end

app.get '/', handler_name: 'get_root' do |request|
  { status: 'ok' }
end

# ============================================================================
# Server Startup
# ============================================================================

if __FILE__ == $0
  port = (ARGV[0] || 8000).to_i
  $stderr.puts "[spikard-ruby] Starting server on port #{port}"
  app.run(host: '0.0.0.0', port: port)
end
