#!/usr/bin/env ruby
# frozen_string_literal: true

# Roda HTTP server for benchmark testing.
#
# This server implements two modes via path prefixes:
# - Raw endpoints (original paths): No validation, parameters returned as-is
# - Validated endpoints (/validated/...): Full validation with Dry::Schema
#
# This allows measuring both raw performance and validation overhead.

require 'roda'
require 'dry/schema'
require 'json'
require 'date'

# ============================================================================
# Validation Schemas
# ============================================================================

# JSON Body Schemas (matching benchmark schemas)
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

LargePayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:price).filled(:float)
  required(:seller).hash(SellerSchema)
end

VeryLargePayloadSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:tags).array(:str?)
  required(:images).array(ImageSchema)
end

# URL-Encoded Form Schemas
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
# Roda Application - Raw + Validated
# ============================================================================

class BenchmarkApp < Roda
  plugin :json
  plugin :json_parser
  plugin :all_verbs
  plugin :type_routing
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
    # ========== RAW ENDPOINTS (No Validation) ==========

    # JSON body endpoints - just parse and echo back
    r.on 'json' do
      r.post 'small' do
        r.params
      end

      r.post 'medium' do
        r.params
      end

      r.post 'large' do
        r.params
      end

      r.post 'very-large' do
        r.params
      end
    end

    # Multipart form endpoints - static responses
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

    # URL-encoded form endpoints - just echo params
    r.on 'urlencoded' do
      r.post 'simple' do
        r.params
      end

      r.post 'complex' do
        r.params
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

      r.get 'int', Integer do |id|
        { id: id }
      end

      r.get 'uuid', String do |uuid|
        { uuid: uuid }
      end

      r.get 'date', String do |date|
        begin
          Date.iso8601(date)
        rescue ArgumentError
          # In raw mode, still convert but don't strictly validate
        end
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

    # ========== VALIDATED ENDPOINTS (/validated/... prefix) ==========

    # JSON body endpoints (4 endpoints)
    r.on 'validated' do
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
          { id: id }
        end

        r.get 'uuid', String do |uuid|
          unless uuid =~ /\A[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\z/i
            response.status = 400
            next { error: 'Invalid UUID format' }
          end
          { uuid: uuid }
        end

        r.get 'date', String do |date|
          begin
            Date.iso8601(date)
          rescue ArgumentError
            response.status = 400
            next { error: 'Invalid date format' }
          end
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

  handler = Rackup::Handler.get('falcon')
  handler.run(
    BenchmarkApp.freeze.app,
    Port: port,
    Host: '0.0.0.0'
  )
end
