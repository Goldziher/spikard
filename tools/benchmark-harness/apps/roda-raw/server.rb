#!/usr/bin/env ruby
# frozen_string_literal: true

# Roda HTTP server for raw performance benchmarking (NO VALIDATION).
#
# This server implements all workload types WITHOUT validation to measure
# Roda's raw performance overhead without Dry::Schema validation costs.

require 'roda'
require 'json'

# ============================================================================
# Roda Application - Raw Performance (No Validation)
# ============================================================================

class BenchmarkApp < Roda
  plugin :json
  plugin :json_parser
  plugin :all_verbs
  plugin :type_routing

  route do |r|
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
  $stderr.puts "[roda-raw] Starting server on port #{port}"

  handler = Rackup::Handler.get('puma')
  handler.run(
    BenchmarkApp.freeze.app,
    Port: port,
    Host: '0.0.0.0',
    Threads: '1:1',
    Silent: true
  )
end
