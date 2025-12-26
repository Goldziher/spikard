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
    app = Spikard::App.new

    # Health check endpoint
    app.get '/health' do |_req|
      { status: 'ok' }
    end

    # Query parameters endpoint
    app.get '/query' do |req|
      {
        name: req.query['name'],
        age: req.query['age']&.to_i
      }
    end

    # JSON echo endpoint
    app.post '/echo' do |req|
      body = req.body
      {
        received: body,
        method: req.method
      }
    end

    # Path parameters endpoint
    app.get '/users/:id' do |req|
      user_id = req.path_params['id']
      {
        userId: user_id,
        type: user_id.class.to_s
      }
    end

    app
  end
end
