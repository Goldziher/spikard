#!/usr/bin/env ruby
# frozen_string_literal: true

# Roda HTTP server for workload benchmarking.
#
# This server implements all 18 workload endpoints to measure Roda performance
# against spikard-ruby and other Ruby frameworks.

require 'roda'
require 'dry/schema'
require 'json'

# ============================================================================
# Validation Schemas
# ============================================================================

# JSON Body Schemas (matching spikard-ruby workload format)
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
  required(:tags).array(:string)
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
  required(:data).array(:hash)
  required(:metadata).hash
end

# URL-Encoded Form Schemas
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
  plugin :json_parser
  plugin :all_verbs
  plugin :halt

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
    # JSON body endpoints (4 endpoints)
    r.on 'json' do
      r.post 'small' do
        validate_and_respond(SmallPayloadSchema, r.params)
      end

      r.post 'medium' do
        validate_and_respond(MediumPayloadSchema, r.params)
      end

      r.post 'large' do
        result = LargePayloadSchema.call(r.params)
        unless result.success?
          response.status = 400
          next { errors: result.errors.to_h }
        end
        result.to_h
      end

      r.post 'very-large' do
        result = VeryLargePayloadSchema.call(r.params)
        unless result.success?
          response.status = 400
          next { errors: result.errors.to_h }
        end
        result.to_h
      end
    end

    # Path parameter endpoints (6 endpoints)
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

      r.get 'int', Integer do |id|
        { id: id, type: 'integer' }
      end

      r.get 'uuid', String do |uuid|
        { uuid: uuid }
      end

      r.get 'date', String do |date|
        { date: date }
      end
    end

    # Query parameter endpoints (3 endpoints)
    r.on 'query' do
      r.get 'few' do
        {
          page: r.params['page'],
          limit: r.params['limit']
        }
      end

      r.get 'medium' do
        {
          page: r.params['page'],
          limit: r.params['limit'],
          sort: r.params['sort'],
          order: r.params['order'],
          filter: r.params['filter']
        }
      end

      r.get 'many' do
        {
          page: r.params['page'],
          limit: r.params['limit'],
          sort: r.params['sort'],
          order: r.params['order'],
          filter: r.params['filter'],
          search: r.params['search'],
          category: r.params['category'],
          tag: r.params['tag'],
          status: r.params['status'],
          priority: r.params['priority']
        }
      end
    end

    # URL-encoded form endpoints (2 endpoints)
    r.on 'urlencoded' do
      r.post 'simple' do
        validate_and_respond(UrlencodedSimpleSchema, r.params)
      end

      r.post 'complex' do
        validate_and_respond(UrlencodedComplexSchema, r.params)
      end
    end

    # Multipart form endpoints (3 endpoints)
    r.on 'multipart' do
      r.post 'small' do
        file = r.params['file']
        name = r.params['name']

        unless file && file.is_a?(Hash) && file[:tempfile]
          response.status = 400
          next { error: 'File upload required' }
        end

        {
          status: 'ok',
          name: name,
          filename: file[:filename],
          size: file[:tempfile].size
        }
      end

      r.post 'medium' do
        file = r.params['file']
        description = r.params['description']
        tags = r.params['tags']

        unless file && file.is_a?(Hash) && file[:tempfile]
          response.status = 400
          next { error: 'File upload required' }
        end

        {
          status: 'ok',
          filename: file[:filename],
          size: file[:tempfile].size,
          description: description,
          tags: tags
        }
      end

      r.post 'large' do
        files = []
        metadata = r.params['metadata']

        # Handle multiple files
        if r.params['files']
          files_param = r.params['files']
          if files_param.is_a?(Array)
            files = files_param.select { |f| f.is_a?(Hash) && f[:tempfile] }
          elsif files_param.is_a?(Hash) && files_param[:tempfile]
            files = [files_param]
          end
        end

        {
          status: 'ok',
          file_count: files.length,
          total_size: files.sum { |f| f[:tempfile].size },
          metadata: metadata
        }
      end
    end

    # Health check endpoint
    r.get 'health' do
      { status: 'ok' }
    end

    r.root do
      { status: 'ok', framework: 'roda', version: Roda::RodaVersion }
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
