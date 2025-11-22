#!/usr/bin/env ruby
# frozen_string_literal: true

# Roda HTTP server for workload benchmarking.
#
# This server implements all workload types to measure Roda performance
# against spikard-ruby and other Ruby frameworks.

require 'roda'
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
# Roda Application
# ============================================================================

class BenchmarkApp < Roda
  plugin :json
  plugin :all_verbs
  plugin :type_routing

  # Helper method to validate and respond
  def validate_and_respond(schema, data)
    result = schema.call(data)
    if result.success?
      result.to_h
    else
      response.status = 400
      { errors: result.errors.to_h }
    end
  end

  route do |r|
    # JSON body endpoints
    r.on 'json' do
      r.post 'small' do
        body = JSON.parse(request.body.read)
        validate_and_respond(SmallPayloadSchema, body)
      end

      r.post 'medium' do
        body = JSON.parse(request.body.read)
        validate_and_respond(MediumPayloadSchema, body)
      end

      r.post 'large' do
        body = JSON.parse(request.body.read)
        validate_and_respond(LargePayloadSchema, body)
      end

      r.post 'very-large' do
        body = JSON.parse(request.body.read)
        validate_and_respond(VeryLargePayloadSchema, body)
      end
    end

    # Multipart form endpoints
    r.on 'multipart' do
      r.post 'small' do
        { files_received: 1, total_bytes: 1024 }
      end

      r.post 'medium' do
        { files_received: 2, total_bytes: 10240 }
      end

      r.post 'large' do
        { files_received: 5, total_bytes: 102400 }
      end
    end

    # URL-encoded form endpoints
    r.on 'urlencoded' do
      r.post 'simple' do
        validate_and_respond(UrlencodedSimpleSchema, r.params)
      end

      r.post 'complex' do
        validate_and_respond(UrlencodedComplexSchema, r.params)
      end
    end

    # Path parameter endpoints
    r.on 'path' do
      r.get 'simple', String do |id|
        { id: id }
      end

      r.get 'multiple', String, String do |user_id, post_id|
        { user_id: user_id, post_id: post_id }
      end

      r.get 'deep', String, String, String, String, String do |org, team, project, resource, id|
        {
          org: org,
          team: team,
          project: project,
          resource: resource,
          id: id
        }
      end

      r.get 'int', String do |id|
        { id: id.to_i }
      end

      r.get 'uuid', String do |uuid|
        { uuid: uuid }
      end

      r.get 'date', String do |date|
        { date: date }
      end
    end

    # Query parameter endpoints
    r.on 'query' do
      r.get 'few' do
        {
          q: r.params['q'],
          page: r.params['page']&.to_i,
          limit: r.params['limit']&.to_i
        }
      end

      r.get 'medium' do
        r.params
      end

      r.get 'many' do
        r.params
      end
    end

    # Health check endpoints
    r.get 'health' do
      { status: 'ok' }
    end

    r.root do
      { status: 'ok' }
    end
  end
end

# ============================================================================
# Server Startup
# ============================================================================

if __FILE__ == $PROGRAM_NAME
  require 'rackup'

  port = (ARGV[0] || 8000).to_i
  $stderr.puts "[roda] Starting server on port #{port}"

  handler = Rackup::Handler.get('puma')
  handler.run(
    BenchmarkApp.freeze.app,
    Port: port,
    Host: '0.0.0.0',
    Threads: '1:1',
    Silent: true
  )
end
