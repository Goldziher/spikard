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
# - HTTP methods (PUT, DELETE, PATCH)
# - Headers and cookies
# - Error handling
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

    # PUT items endpoint
    app.put '/items/:id' do |req|
      item_id = req.path_params['id']
      body = req.body
      {
        itemId: item_id,
        updated: body,
        method: req.method
      }
    end

    # DELETE items endpoint
    app.delete '/items/:id' do |req|
      item_id = req.path_params['id']
      {
        itemId: item_id,
        deleted: true,
        method: req.method
      }
    end

    # PATCH items endpoint
    app.patch '/items/:id' do |req|
      item_id = req.path_params['id']
      body = req.body
      {
        itemId: item_id,
        patched: body,
        method: req.method
      }
    end

    # Headers endpoint
    app.get '/headers' do |req|
      {
        'x-custom-header': req.headers['x-custom-header']
      }
    end

    # Cookies endpoint
    app.get '/cookies' do |req|
      {
        session: req.cookies['session']
      }
    end

    # Error endpoint
    app.get '/error' do |_req|
      raise 'Intentional error'
    end

    app
  end
end
