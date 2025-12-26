#!/usr/bin/env ruby
# frozen_string_literal: true

require 'spikard'
require 'json'

# Ruby test application for Spikard
#
# Tests core functionality:
# - Health check endpoint
# - Query parameter handling
# - JSON request/response
# - Path parameter extraction
module SpikardTestApp
  def self.create_app
    server = Spikard::Server.new(host: '127.0.0.1', port: 0)

    # Health check
    server.get '/health' do |_req|
      {
        status: 200,
        headers: { 'Content-Type' => 'application/json' },
        body: JSON.generate({ status: 'ok' })
      }
    end

    # Query parameters
    server.get '/query' do |req|
      params = req.query_params || {}
      {
        status: 200,
        headers: { 'Content-Type' => 'application/json' },
        body: JSON.generate({
          name: params['name'],
          age: params['age']&.to_i
        })
      }
    end

    # JSON echo
    server.post '/echo' do |req|
      body = req.body ? JSON.parse(req.body) : {}
      {
        status: 200,
        headers: { 'Content-Type' => 'application/json' },
        body: JSON.generate({
          received: body,
          method: req.method
        })
      }
    end

    # Path parameters
    server.get '/users/:id' do |req|
      user_id = req.path_params&.fetch('id', nil)
      {
        status: 200,
        headers: { 'Content-Type' => 'application/json' },
        body: JSON.generate({
          userId: user_id,
          type: user_id.class.to_s
        })
      }
    end

    server
  end
end
