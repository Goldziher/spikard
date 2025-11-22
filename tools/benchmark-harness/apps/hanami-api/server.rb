#!/usr/bin/env ruby
# frozen_string_literal: true

# Hanami API HTTP server for workload benchmarking.
#
# This server implements all workload types to measure Hanami API performance
# against spikard-ruby and other Ruby frameworks.

require 'hanami/api'
require 'dry/schema'
require 'json'

# ============================================================================
# Validation Schemas
# ============================================================================

SmallPayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:description).filled(:string)
  required(:price).filled(:float)
  optional(:tax).maybe(:float)
end

AddressSchema = Dry::Schema.JSON do
  required(:street).filled(:string)
  required(:city).filled(:string)
  required(:state).filled(:string)
  required(:zip_code).filled(:string)
end

MediumPayloadSchema = Dry::Schema.JSON do
  required(:user_id).filled(:integer)
  required(:username).filled(:string)
  required(:email).filled(:string)
  required(:is_active).filled(:bool)
  required(:address).hash(AddressSchema)
  required(:tags).filled(:array)
end

ItemSchema = Dry::Schema.JSON do
  required(:id).filled(:integer)
  required(:name).filled(:string)
  required(:price).filled(:float)
  required(:in_stock).filled(:bool)
end

LargePayloadSchema = Dry::Schema.JSON do
  required(:order_id).filled(:string)
  required(:customer_name).filled(:string)
  required(:items).array(ItemSchema)
  required(:total).filled(:float)
  required(:notes).filled(:string)
end

VeryLargePayloadSchema = Dry::Schema.JSON do
  required(:data).filled(:array)
  required(:metadata).filled(:hash)
end

UrlencodedSimpleSchema = Dry::Schema.Params do
  required(:name).filled(:string)
  optional(:email).maybe(:string)
end

UrlencodedComplexSchema = Dry::Schema.Params do
  required(:user).hash do
    required(:name).filled(:string)
    required(:email).filled(:string)
    optional(:age).maybe(:integer)
  end
  required(:preferences).hash do
    required(:theme).filled(:string)
    optional(:notifications).maybe(:bool)
  end
end

# ============================================================================
# Hanami API Application
# ============================================================================

class BenchmarkApp < Hanami::API
  # JSON body endpoints
  post '/json/small' do
    body = JSON.parse(request.body.read)
    result = SmallPayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/json/medium' do
    body = JSON.parse(request.body.read)
    result = MediumPayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/json/large' do
    body = JSON.parse(request.body.read)
    result = LargePayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  post '/json/very-large' do
    body = JSON.parse(request.body.read)
    result = VeryLargePayloadSchema.call(body)

    if result.success?
      json(result.to_h)
    else
      halt 400, json({ errors: result.errors.to_h })
    end
  end

  # Multipart form endpoints (stub implementations)
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
    json({ id: params[:id].to_i })
  end

  get '/path/uuid/:uuid' do
    json({ uuid: params[:uuid] })
  end

  get '/path/date/:date' do
    json({ date: params[:date] })
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
