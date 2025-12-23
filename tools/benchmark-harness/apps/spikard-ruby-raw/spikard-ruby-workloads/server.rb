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

app.get '/path/deep/:org/:team/:project/:resource/:id' do |params|
  {
    org: params[:org],
    team: params[:team],
    project: params[:project],
    resource: params[:resource],
    id: params[:id]
  }
end

app.get '/path/int/:id' do |params|
  { id: params[:id].to_i }
end

app.get '/path/uuid/:uuid' do |params|
  { uuid: params[:uuid] }
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
