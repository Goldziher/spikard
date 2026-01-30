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

    # Multipart form endpoints - parse actual uploaded files
    r.on 'multipart' do
      r.post 'small' do
        files_received = 0
        total_bytes = 0

        r.params.each do |key, value|
          if value.is_a?(Hash) && value[:tempfile]
            files_received += 1
            total_bytes += value[:tempfile].size
          end
        end

        { files_received: files_received, total_bytes: total_bytes }
      end

      r.post 'medium' do
        files_received = 0
        total_bytes = 0

        r.params.each do |key, value|
          if value.is_a?(Hash) && value[:tempfile]
            files_received += 1
            total_bytes += value[:tempfile].size
          end
        end

        { files_received: files_received, total_bytes: total_bytes }
      end

      r.post 'large' do
        files_received = 0
        total_bytes = 0

        r.params.each do |key, value|
          if value.is_a?(Hash) && value[:tempfile]
            files_received += 1
            total_bytes += value[:tempfile].size
          end
        end

        { files_received: files_received, total_bytes: total_bytes }
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
          unless id =~ /\A[a-zA-Z0-9_-]+\z/ && !id.empty? && id.length <= 255
            response.status = 400
            next { error: 'Invalid path parameter format' }
          end
          { id: id }
        end

        r.get 'multiple', String, String do |user_id, post_id|
          unless user_id =~ /\A[a-zA-Z0-9_-]+\z/ && !user_id.empty? && user_id.length <= 255
            response.status = 400
            next { error: 'Invalid path parameter format' }
          end
          unless post_id =~ /\A[a-zA-Z0-9_-]+\z/ && !post_id.empty? && post_id.length <= 255
            response.status = 400
            next { error: 'Invalid path parameter format' }
          end
          { user_id: user_id, post_id: post_id }
        end

        r.get 'deep', String, String, String, String, String do |org, team, project, resource, id|
          params_hash = { org: org, team: team, project: project, resource: resource, id: id }
          invalid = params_hash.find { |_, v| v.nil? || v.empty? || v.length > 255 || v !~ /\A[a-zA-Z0-9_-]+\z/ }
          if invalid
            response.status = 400
            response.write(JSON.generate({ error: "Invalid path parameter: #{invalid[0]}" }))
            request.halt
          end
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
          validate_and_respond(QueryFewSchema, r.params)
        end

        r.get 'medium' do
          validate_and_respond(QueryMediumSchema, r.params)
        end

        r.get 'many' do
          validate_and_respond(QueryManySchema, r.params)
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

      # Multipart form endpoints (3 endpoints) - parse and validate
      r.on 'multipart' do
        r.post 'small' do
          files_received = 0
          total_bytes = 0

          r.params.each do |key, value|
            if value.is_a?(Hash) && value[:tempfile]
              files_received += 1
              total_bytes += value[:tempfile].size
            end
          end

          if files_received == 0
            response.status = 400
            next { error: 'No files uploaded' }
          end

          { files_received: files_received, total_bytes: total_bytes }
        end

        r.post 'medium' do
          files_received = 0
          total_bytes = 0

          r.params.each do |key, value|
            if value.is_a?(Hash) && value[:tempfile]
              files_received += 1
              total_bytes += value[:tempfile].size
            end
          end

          if files_received == 0
            response.status = 400
            next { error: 'No files uploaded' }
          end

          { files_received: files_received, total_bytes: total_bytes }
        end

        r.post 'large' do
          files_received = 0
          total_bytes = 0

          r.params.each do |key, value|
            if value.is_a?(Hash) && value[:tempfile]
              files_received += 1
              total_bytes += value[:tempfile].size
            end
          end

          if files_received == 0
            response.status = 400
            next { error: 'No files uploaded' }
          end

          { files_received: files_received, total_bytes: total_bytes }
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
