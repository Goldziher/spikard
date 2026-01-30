#!/usr/bin/env ruby
# frozen_string_literal: true

# Hanami API HTTP server for benchmark testing.
#
# This server implements two modes via path prefixes:
# - Raw endpoints (original paths): No validation, JSON parsed and echoed back
# - Validated endpoints (/validated/...): Full validation with Dry::Schema
#
# This allows measuring both raw performance and validation overhead.

require 'hanami/api'
require 'dry/schema'
require 'json'
require 'date'

# ============================================================================
# Validation Schemas (Dry::Schema.JSON matching Python Pydantic patterns)
# ============================================================================

# Small JSON payload schema (~100 bytes)
SmallPayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:description).filled(:string)
  required(:price).filled(:float)
  required(:tax).filled(:float)
end

ImageSchema = Dry::Schema.JSON do
  required(:url).filled(:string)
  required(:name).filled(:string)
end

# Medium JSON payload schema (~1KB)
MediumPayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:price).filled(:float)
  required(:image).hash(ImageSchema)
end

CountrySchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:code).filled(:string)
end

SellerAddressSchema = Dry::Schema.JSON do
  required(:street).filled(:string)
  required(:city).filled(:string)
  required(:country).hash(CountrySchema)
end

SellerSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:address).hash(SellerAddressSchema)
end

# Large JSON payload schema (~10KB)
LargePayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:price).filled(:float)
  required(:seller).hash(SellerSchema)
end

# Very large JSON payload schema (~100KB)
VeryLargePayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:tags).array(:str?)
  required(:images).array(ImageSchema)
end

# URL-encoded form schemas
UrlencodedSimpleSchema = Dry::Schema.Params do
  required(:name).filled(:string)
  required(:email).filled(:string)
  required(:age).filled(:integer)
  required(:subscribe).filled(:bool)
end

UrlencodedComplexSchema = Dry::Schema.Params do
  required(:username).filled(:string)
  required(:password).filled(:string)
  required(:email).filled(:string)
  required(:first_name).filled(:string)
  required(:last_name).filled(:string)
  required(:age).filled(:integer)
  required(:country).filled(:string)
  required(:state).filled(:string)
  required(:city).filled(:string)
  required(:zip).filled(:string)
  required(:phone).filled(:string)
  required(:company).filled(:string)
  required(:job_title).filled(:string)
  required(:subscribe).filled(:bool)
  required(:newsletter).filled(:bool)
  required(:terms_accepted).filled(:bool)
  required(:privacy_accepted).filled(:bool)
  required(:marketing_consent).filled(:bool)
  required(:two_factor_enabled).filled(:bool)
end

# Query parameter schemas
QueryFewSchema = Dry::Schema.Params do
  required(:q).filled(:string)
  optional(:page).filled(:integer)
  optional(:limit).filled(:integer)
end

QueryMediumSchema = Dry::Schema.Params do
  required(:search).filled(:string)
  optional(:category).filled(:string)
  optional(:sort).filled(:string)
  optional(:order).filled(:string)
  optional(:page).filled(:integer)
  optional(:limit).filled(:integer)
  optional(:filter).filled(:string)
end

QueryManySchema = Dry::Schema.Params do
  required(:q).filled(:string)
  optional(:category).filled(:string)
  optional(:subcategory).filled(:string)
  optional(:brand).filled(:string)
  optional(:min_price).filled(:float)
  optional(:max_price).filled(:float)
  optional(:color).filled(:string)
  optional(:size).filled(:string)
  optional(:material).filled(:string)
  optional(:rating).filled(:integer)
  optional(:sort).filled(:string)
  optional(:order).filled(:string)
  optional(:page).filled(:integer)
  optional(:limit).filled(:integer)
  optional(:in_stock).filled(:bool)
  optional(:on_sale).filled(:bool)
end

# ============================================================================
# Hanami API Application (Raw + Validated)
# ============================================================================

class BenchmarkApp < Hanami::API
  # ========== RAW ENDPOINTS (No Validation) ==========

  # JSON body endpoints - parse and echo without validation
  post '/json/small' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  post '/json/medium' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  post '/json/large' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  post '/json/very-large' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    json(body)
  end

  # Multipart form endpoints - parse actual uploaded files
  post '/multipart/small' do
    request = Rack::Request.new(env)
    files_received = 0
    total_bytes = 0

    request.params.each do |key, value|
      if value.is_a?(Hash) && value[:tempfile]
        files_received += 1
        total_bytes += value[:tempfile].size
      end
    end

    json({ files_received: files_received, total_bytes: total_bytes })
  end

  post '/multipart/medium' do
    request = Rack::Request.new(env)
    files_received = 0
    total_bytes = 0

    request.params.each do |key, value|
      if value.is_a?(Hash) && value[:tempfile]
        files_received += 1
        total_bytes += value[:tempfile].size
      end
    end

    json({ files_received: files_received, total_bytes: total_bytes })
  end

  post '/multipart/large' do
    request = Rack::Request.new(env)
    files_received = 0
    total_bytes = 0

    request.params.each do |key, value|
      if value.is_a?(Hash) && value[:tempfile]
        files_received += 1
        total_bytes += value[:tempfile].size
      end
    end

    json({ files_received: files_received, total_bytes: total_bytes })
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

  # ========== VALIDATED ENDPOINTS (/validated/... prefix) ==========

  # JSON body endpoints - validate and echo back
  post '/validated/json/small' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    result = SmallPayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/validated/json/medium' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    result = MediumPayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/validated/json/large' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    result = LargePayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/validated/json/very-large' do
    env['rack.input'].rewind
    body = JSON.parse(env['rack.input'].read)
    result = VeryLargePayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  # Multipart form endpoints - parse and validate actual uploaded files
  post '/validated/multipart/small' do
    request = Rack::Request.new(env)
    files_received = 0
    total_bytes = 0

    request.params.each do |key, value|
      if value.is_a?(Hash) && value[:tempfile]
        files_received += 1
        total_bytes += value[:tempfile].size
      end
    end

    if files_received == 0
      halt 400, json({ error: 'No files uploaded' })
    end

    json({ files_received: files_received, total_bytes: total_bytes })
  end

  post '/validated/multipart/medium' do
    request = Rack::Request.new(env)
    files_received = 0
    total_bytes = 0

    request.params.each do |key, value|
      if value.is_a?(Hash) && value[:tempfile]
        files_received += 1
        total_bytes += value[:tempfile].size
      end
    end

    if files_received == 0
      halt 400, json({ error: 'No files uploaded' })
    end

    json({ files_received: files_received, total_bytes: total_bytes })
  end

  post '/validated/multipart/large' do
    request = Rack::Request.new(env)
    files_received = 0
    total_bytes = 0

    request.params.each do |key, value|
      if value.is_a?(Hash) && value[:tempfile]
        files_received += 1
        total_bytes += value[:tempfile].size
      end
    end

    if files_received == 0
      halt 400, json({ error: 'No files uploaded' })
    end

    json({ files_received: files_received, total_bytes: total_bytes })
  end

  # URL-encoded form endpoints
  post '/validated/urlencoded/simple' do
    result = UrlencodedSimpleSchema.call(params)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/validated/urlencoded/complex' do
    result = UrlencodedComplexSchema.call(params)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  # Path parameter endpoints - validate string params
  get '/validated/path/simple/:id' do
    id = params[:id]
    unless id =~ /\A[a-zA-Z0-9_-]+\z/ && !id.empty? && id.length <= 255
      halt 400, json({ error: 'Invalid path parameter format' })
    end
    json({ id: id })
  end

  get '/validated/path/multiple/:user_id/:post_id' do
    user_id = params[:user_id]
    post_id = params[:post_id]
    unless user_id =~ /\A[a-zA-Z0-9_-]+\z/ && !user_id.empty? && user_id.length <= 255
      halt 400, json({ error: 'Invalid path parameter format' })
    end
    unless post_id =~ /\A[a-zA-Z0-9_-]+\z/ && !post_id.empty? && post_id.length <= 255
      halt 400, json({ error: 'Invalid path parameter format' })
    end
    json({
      user_id: user_id,
      post_id: post_id
    })
  end

  get '/validated/path/deep/:org/:team/:project/:resource/:id' do
    org = params[:org]
    team = params[:team]
    project = params[:project]
    resource = params[:resource]
    id = params[:id]
    [org, team, project, resource, id].each do |param|
      unless param =~ /\A[a-zA-Z0-9_-]+\z/ && !param.empty? && param.length <= 255
        halt 400, json({ error: 'Invalid path parameter format' })
      end
    end
    json({
      org: org,
      team: team,
      project: project,
      resource: resource,
      id: id
    })
  end

  get '/validated/path/int/:id' do
    id_str = params[:id]
    unless id_str =~ /\A-?\d+\z/
      halt 400, json({ error: 'Invalid integer format' })
    end
    json({ id: id_str.to_i })
  end

  get '/validated/path/uuid/:uuid' do
    uuid = params[:uuid]
    unless uuid =~ /\A[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\z/i
      halt 400, json({ error: 'Invalid UUID format' })
    end
    json({ uuid: uuid })
  end

  get '/validated/path/date/:date' do
    date_str = params[:date]
    begin
      Date.iso8601(date_str)
      json({ date: date_str })
    rescue ArgumentError
      halt 400, json({ error: 'Invalid date format' })
    end
  end

  # Query parameter endpoints - validate and return query params as JSON
  get '/validated/query/few' do
    result = QueryFewSchema.call(params)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  get '/validated/query/medium' do
    result = QueryMediumSchema.call(params)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  get '/validated/query/many' do
    result = QueryManySchema.call(params)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
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
  $stderr.puts "[hanami-api] Starting server on port #{port}"

  handler = Rackup::Handler.get('puma')
  handler.run(
    BenchmarkApp.new,
    Port: port,
    Host: '0.0.0.0',
    Threads: '4:16',
    Silent: true
  )
end
