#!/usr/bin/env ruby
# frozen_string_literal: true

# Hanami API HTTP server for workload benchmarking.
#
# This server implements all workload types to measure Hanami API performance
# against spikard-ruby and other Ruby frameworks using Dry::Schema for validation.

require 'hanami/api'
require 'dry/schema'
require 'json'

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

# ============================================================================
# Hanami API Application
# ============================================================================

class BenchmarkApp < Hanami::API
  # JSON body endpoints - validate and echo back
  post '/json/small' do
    body = JSON.parse(env['rack.input'].read)
    result = SmallPayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/json/medium' do
    body = JSON.parse(env['rack.input'].read)
    result = MediumPayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/json/large' do
    body = JSON.parse(env['rack.input'].read)
    result = LargePayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/json/very-large' do
    body = JSON.parse(env['rack.input'].read)
    result = VeryLargePayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  # Multipart form endpoints (stub implementations - return expected format)
  post '/multipart/small' do
    json({ files_received: 1, total_bytes: 1024 })
  end

  post '/multipart/medium' do
    json({ files_received: 2, total_bytes: 10240 })
  end

  post '/multipart/large' do
    json({ files_received: 5, total_bytes: 102400 })
  end

  # URL-encoded form endpoints
  post '/urlencoded/simple' do
    result = UrlencodedSimpleSchema.call(params)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/urlencoded/complex' do
    result = UrlencodedComplexSchema.call(params)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  # Path parameter endpoints - extract and return params
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
    json({ id: params[:id].to_i })
  end

  get '/path/uuid/:uuid' do
    json({ uuid: params[:uuid] })
  end

  get '/path/date/:date' do
    json({ date: params[:date] })
  end

  # Query parameter endpoints - return query params as JSON
  get '/query/few' do
    result = {}
    result[:q] = params[:q] if params[:q]
    result[:page] = params[:page].to_i if params[:page]
    result[:limit] = params[:limit].to_i if params[:limit]
    json(result)
  end

  get '/query/medium' do
    # Return all query params from request
    query_string = env['QUERY_STRING'] || ''
    result = {}
    query_string.split('&').each do |pair|
      key, value = pair.split('=', 2)
      result[key] = value if key && value
    end
    json(result)
  end

  get '/query/many' do
    # Return all query params from request
    query_string = env['QUERY_STRING'] || ''
    result = {}
    query_string.split('&').each do |pair|
      key, value = pair.split('=', 2)
      result[key] = value if key && value
    end
    json(result)
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
    Threads: '1:1',
    Silent: true
  )
end
