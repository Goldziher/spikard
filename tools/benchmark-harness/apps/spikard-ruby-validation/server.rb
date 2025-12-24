#!/usr/bin/env ruby
# frozen_string_literal: true

# Spikard Ruby HTTP server for workload benchmarking.
#
# This server implements all workload types to measure Ruby binding performance.

$LOAD_PATH.unshift File.expand_path('../../../../packages/ruby/lib', __dir__)
require 'spikard_rb'
require 'spikard'
require 'json'

PROFILE_ENABLED = ENV.fetch('SPIKARD_PROFILE_ENABLED', '0') == '1'

GC::Profiler.enable if PROFILE_ENABLED

def write_benchmark_metrics
  return unless PROFILE_ENABLED

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

if PROFILE_ENABLED
  Signal.trap('TERM') { write_benchmark_metrics; exit(0) }
  Signal.trap('INT') { write_benchmark_metrics; exit(0) }
  at_exit { write_benchmark_metrics }
end

app = Spikard::App.new

# ============================================================================
# Shared Schema Loading
# ============================================================================

schema_dir = File.expand_path('../schemas', __dir__)
REQUEST_SCHEMAS = JSON.parse(File.read(File.join(schema_dir, 'request_schemas.json')))
PARAMETER_SCHEMAS = JSON.parse(File.read(File.join(schema_dir, 'parameter_schemas.json')))
RESPONSE_SCHEMAS = JSON.parse(File.read(File.join(schema_dir, 'response_schemas.json')))

def request_schema(key)
  REQUEST_SCHEMAS.fetch(key)
end

def parameter_schema(key)
  PARAMETER_SCHEMAS.fetch(key)
end

def response_schema(key)
  RESPONSE_SCHEMAS.fetch(key)
end

def normalize_json(value)
  case value
  when Hash
    value.each_with_object({}) do |(key, val), output|
      output[key.to_s] = normalize_json(val)
    end
  when Array
    value.map { |item| normalize_json(item) }
  else
    value
  end
end

def extract_body(payload)
  return {} if payload.nil?

  if payload.is_a?(Hash)
    body = payload.fetch(:body, payload.fetch('body', payload))
    return normalize_json(body)
  end

  if payload.respond_to?(:to_h)
    body = payload.to_h.fetch(:body, payload.to_h.fetch('body', payload.to_h))
    return normalize_json(body)
  end

  normalize_json(payload)
end

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
# Schema Definitions
# ============================================================================

SMALL_PAYLOAD_SCHEMA = {
  "type" => "object",
  "required" => %w[name description price tax],
  "properties" => {
    "name" => { "type" => "string" },
    "description" => { "type" => "string" },
    "price" => { "type" => "number" },
    "tax" => { "type" => "number" }
  },
  "additionalProperties" => false
}.freeze

MEDIUM_PAYLOAD_SCHEMA = {
  "type" => "object",
  "required" => %w[name price image],
  "properties" => {
    "name" => { "type" => "string" },
    "price" => { "type" => "number" },
    "image" => {
      "type" => "object",
      "required" => %w[url name],
      "properties" => {
        "url" => { "type" => "string" },
        "name" => { "type" => "string" }
      },
      "additionalProperties" => false
    }
  },
  "additionalProperties" => false
}.freeze

LARGE_PAYLOAD_SCHEMA = {
  "type" => "object",
  "required" => %w[name price seller],
  "properties" => {
    "name" => { "type" => "string" },
    "price" => { "type" => "number" },
    "seller" => {
      "type" => "object",
      "required" => %w[name address],
      "properties" => {
        "name" => { "type" => "string" },
        "address" => {
          "type" => "object",
          "required" => %w[street city country],
          "properties" => {
            "street" => { "type" => "string" },
            "city" => { "type" => "string" },
            "country" => {
              "type" => "object",
              "required" => %w[name code],
              "properties" => {
                "name" => { "type" => "string" },
                "code" => { "type" => "string" }
              },
              "additionalProperties" => false
            }
          },
          "additionalProperties" => false
        }
      },
      "additionalProperties" => false
    }
  },
  "additionalProperties" => false
}.freeze

VERY_LARGE_PAYLOAD_SCHEMA = {
  "type" => "object",
  "required" => %w[name tags images],
  "properties" => {
    "name" => { "type" => "string" },
    "tags" => {
      "type" => "array",
      "items" => { "type" => "string" }
    },
    "images" => {
      "type" => "array",
      "items" => {
        "type" => "object",
        "required" => %w[url name],
        "properties" => {
          "url" => { "type" => "string" },
          "name" => { "type" => "string" }
        },
        "additionalProperties" => false
      }
    }
  },
  "additionalProperties" => false
}.freeze

URLENCODED_SIMPLE_SCHEMA = {
  "type" => "object",
  "required" => %w[name email age subscribe],
  "properties" => {
    "name" => { "type" => "string" },
    "email" => { "type" => "string", "format" => "email" },
    "age" => { "type" => "integer" },
    "subscribe" => { "type" => "boolean" }
  },
  "additionalProperties" => false
}.freeze

URLENCODED_COMPLEX_SCHEMA = {
  "type" => "object",
  "required" => %w[
    username
    password
    email
    first_name
    last_name
    age
    country
    state
    city
    zip
    phone
    company
    job_title
    subscribe
    newsletter
    terms_accepted
    privacy_accepted
    marketing_consent
    two_factor_enabled
  ],
  "properties" => {
    "username" => { "type" => "string" },
    "password" => { "type" => "string" },
    "email" => { "type" => "string", "format" => "email" },
    "first_name" => { "type" => "string" },
    "last_name" => { "type" => "string" },
    "age" => { "type" => "integer" },
    "country" => { "type" => "string" },
    "state" => { "type" => "string" },
    "city" => { "type" => "string" },
    "zip" => { "type" => "string" },
    "phone" => { "type" => "string" },
    "company" => { "type" => "string" },
    "job_title" => { "type" => "string" },
    "subscribe" => { "type" => "boolean" },
    "newsletter" => { "type" => "boolean" },
    "terms_accepted" => { "type" => "boolean" },
    "privacy_accepted" => { "type" => "boolean" },
    "marketing_consent" => { "type" => "boolean" },
    "two_factor_enabled" => { "type" => "boolean" }
  },
  "additionalProperties" => false
}.freeze

MULTIPART_FILE_SCHEMA = {
  "type" => "object",
  "required" => %w[filename size content content_type],
  "properties" => {
    "filename" => { "type" => "string" },
    "size" => { "type" => "integer" },
    "content" => { "type" => "string" },
    "content_type" => { "type" => "string" }
  },
  "additionalProperties" => false
}.freeze

MULTIPART_SCHEMA = {
  "type" => "object",
  "required" => ["file"],
  "properties" => {
    "file" => {
      "oneOf" => [
        MULTIPART_FILE_SCHEMA,
        { "type" => "array", "items" => MULTIPART_FILE_SCHEMA }
      ]
    }
  },
  "additionalProperties" => false
}.freeze

PATH_SIMPLE_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "id" => { "type" => "string", "source" => "path" }
  },
  "required" => ["id"]
}.freeze

PATH_MULTIPLE_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "user_id" => { "type" => "string", "source" => "path" },
    "post_id" => { "type" => "string", "source" => "path" }
  },
  "required" => %w[user_id post_id]
}.freeze

PATH_DEEP_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "org" => { "type" => "string", "source" => "path" },
    "team" => { "type" => "string", "source" => "path" },
    "project" => { "type" => "string", "source" => "path" },
    "resource" => { "type" => "string", "source" => "path" },
    "id" => { "type" => "string", "source" => "path" }
  },
  "required" => %w[org team project resource id]
}.freeze

PATH_INT_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "id" => { "type" => "integer", "source" => "path" }
  },
  "required" => ["id"]
}.freeze

PATH_UUID_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "uuid" => { "type" => "string", "format" => "uuid", "source" => "path" }
  },
  "required" => ["uuid"]
}.freeze

PATH_DATE_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "date" => { "type" => "string", "format" => "date", "source" => "path" }
  },
  "required" => ["date"]
}.freeze

QUERY_FEW_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "q" => { "type" => "string", "source" => "query" },
    "page" => { "type" => "integer", "source" => "query" },
    "limit" => { "type" => "integer", "source" => "query" }
  },
  "required" => %w[q page limit]
}.freeze

QUERY_MEDIUM_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "category" => { "type" => "string", "source" => "query" },
    "tags" => { "type" => "string", "source" => "query" },
    "min_price" => { "type" => "number", "source" => "query" },
    "max_price" => { "type" => "number", "source" => "query" },
    "sort" => { "type" => "string", "source" => "query" },
    "order" => { "type" => "string", "source" => "query" },
    "page" => { "type" => "integer", "source" => "query" },
    "limit" => { "type" => "integer", "source" => "query" }
  },
  "required" => %w[category tags min_price max_price sort order page limit]
}.freeze

QUERY_MANY_PARAM_SCHEMA = {
  "type" => "object",
  "properties" => {
    "q" => { "type" => "string", "source" => "query" },
    "page" => { "type" => "integer", "source" => "query" },
    "limit" => { "type" => "integer", "source" => "query" },
    "sort" => { "type" => "string", "source" => "query" },
    "order" => { "type" => "string", "source" => "query" },
    "filter" => { "type" => "string", "source" => "query" },
    "category" => { "type" => "string", "source" => "query" },
    "subcategory" => { "type" => "string", "source" => "query" },
    "brand" => { "type" => "string", "source" => "query" },
    "min_price" => { "type" => "number", "source" => "query" },
    "max_price" => { "type" => "number", "source" => "query" },
    "rating" => { "type" => "integer", "source" => "query" },
    "verified" => { "type" => "boolean", "source" => "query" },
    "in_stock" => { "type" => "boolean", "source" => "query" },
    "shipping" => { "type" => "string", "source" => "query" },
    "color" => { "type" => "string", "source" => "query" }
  },
  "required" => %w[
    q
    page
    limit
    sort
    order
    filter
    category
    subcategory
    brand
    min_price
    max_price
    rating
    verified
    in_stock
    shipping
    color
  ]
}.freeze

# ============================================================================
# JSON Body Workloads
# ============================================================================

app.post '/json/small',
         handler_name: 'post_json_small',
         request_schema: request_schema('json/small'),
         response_schema: response_schema('json/small') do |payload = {}|
  extract_body(payload)
end

app.post '/json/medium',
         handler_name: 'post_json_medium',
         request_schema: request_schema('json/medium'),
         response_schema: response_schema('json/medium') do |payload = {}|
  extract_body(payload)
end

app.post '/json/large',
         handler_name: 'post_json_large',
         request_schema: request_schema('json/large'),
         response_schema: response_schema('json/large') do |payload = {}|
  extract_body(payload)
end

app.post '/json/very-large',
         handler_name: 'post_json_very_large',
         request_schema: request_schema('json/very-large'),
         response_schema: response_schema('json/very-large') do |payload = {}|
  extract_body(payload)
end

# ============================================================================
# Multipart Form Workloads
# ============================================================================

app.post '/multipart/small',
         handler_name: 'post_multipart_small',
         request_schema: request_schema('multipart/small'),
         response_schema: response_schema('multipart/small') do |request|
  { files_received: 1, total_bytes: 1024 }
end

app.post '/multipart/medium',
         handler_name: 'post_multipart_medium',
         request_schema: request_schema('multipart/medium'),
         response_schema: response_schema('multipart/medium') do |request|
  { files_received: 2, total_bytes: 10240 }
end

app.post '/multipart/large',
         handler_name: 'post_multipart_large',
         request_schema: request_schema('multipart/large'),
         response_schema: response_schema('multipart/large') do |request|
  { files_received: 5, total_bytes: 102400 }
end

# ============================================================================
# URL Encoded Form Workloads
# ============================================================================

app.post '/urlencoded/simple',
         handler_name: 'post_urlencoded_simple',
         request_schema: request_schema('urlencoded/simple'),
         response_schema: response_schema('urlencoded/simple') do |payload = {}|
  extract_body(payload)
end

app.post '/urlencoded/complex',
         handler_name: 'post_urlencoded_complex',
         request_schema: request_schema('urlencoded/complex'),
         response_schema: response_schema('urlencoded/complex') do |payload = {}|
  extract_body(payload)
end

# ============================================================================
# Path Parameter Workloads
# ============================================================================

app.get '/path/simple/{id}',
        handler_name: 'get_path_simple',
        response_schema: response_schema('path/simple'),
        parameter_schema: parameter_schema('path/simple') do |request|
  { id: request[:path_params][:id] }
end

app.get '/path/multiple/{user_id}/{post_id}',
        handler_name: 'get_path_multiple',
        response_schema: response_schema('path/multiple'),
        parameter_schema: parameter_schema('path/multiple') do |request|
  {
    user_id: request[:path_params][:user_id],
    post_id: request[:path_params][:post_id]
  }
end

app.get '/path/deep/{org}/{team}/{project}/{resource}/{id}',
        handler_name: 'get_path_deep',
        response_schema: response_schema('path/deep'),
        parameter_schema: parameter_schema('path/deep') do |request|
  {
    org: request[:path_params][:org],
    team: request[:path_params][:team],
    project: request[:path_params][:project],
    resource: request[:path_params][:resource],
    id: request[:path_params][:id]
  }
end

app.get '/path/int/{id}',
        handler_name: 'get_path_int',
        response_schema: response_schema('path/int'),
        parameter_schema: parameter_schema('path/int') do |request|
  { id: request[:path_params][:id].to_i }
end

app.get '/path/uuid/{uuid}',
        handler_name: 'get_path_uuid',
        response_schema: response_schema('path/uuid'),
        parameter_schema: parameter_schema('path/uuid') do |request|
  { uuid: request[:path_params][:uuid] }
end

app.get '/path/date/{date}',
        handler_name: 'get_path_date',
        response_schema: response_schema('path/date'),
        parameter_schema: parameter_schema('path/date') do |request|
  { date: request[:path_params][:date] }
end

# ============================================================================
# Query Parameter Workloads
# ============================================================================

app.get '/query/few',
        handler_name: 'get_query_few',
        response_schema: response_schema('query/few'),
        parameter_schema: parameter_schema('query/few') do |request|
  request[:query] || {}
end

app.get '/query/medium',
        handler_name: 'get_query_medium',
        response_schema: response_schema('query/medium'),
        parameter_schema: parameter_schema('query/medium') do |request|
  request[:query] || {}
end

app.get '/query/many',
        handler_name: 'get_query_many',
        response_schema: response_schema('query/many'),
        parameter_schema: parameter_schema('query/many') do |request|
  request[:query] || {}
end

# ============================================================================
# Health Check
# ============================================================================

app.get '/health', handler_name: 'get_health', response_schema: response_schema('health') do |request|
  { status: 'ok' }
end

app.get '/', handler_name: 'get_root', response_schema: response_schema('root') do |request|
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
