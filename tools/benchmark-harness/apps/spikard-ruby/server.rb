#!/usr/bin/env ruby
# frozen_string_literal: true

# Spikard-Ruby benchmark server
#
# Implements all workload types to measure Ruby binding performance.
# Serves both raw endpoints (no validation) and validated endpoints (at /validated/... paths).

$LOAD_PATH.unshift File.expand_path('../../../../packages/ruby/lib', __dir__)
require 'json'
require 'spikard_rb'
require 'spikard'
require 'spikard/handler_wrapper'

app = Spikard::App.new

# Load schemas
schema_dir = File.expand_path('../schemas', __dir__)
request_schemas = JSON.parse(File.read(File.join(schema_dir, 'request_schemas.json')))
parameter_schemas = JSON.parse(File.read(File.join(schema_dir, 'parameter_schemas.json')))
response_schemas = JSON.parse(File.read(File.join(schema_dir, 'response_schemas.json')))

# ===== Handler logic =====

echo_body = lambda { |_params, _query, body| body || {} }

multipart_handler = lambda { |_params, _query, body|
  files = (body.is_a?(Hash) && body['files']) || {}
  files_received = 0
  total_bytes = 0
  files.each do |key, file_data|
    next unless key.start_with?('file') && file_data.is_a?(Hash)

    files_received += 1
    total_bytes += file_data['size'].to_i
  end
  { 'files_received' => files_received, 'total_bytes' => total_bytes }
}

# ===== RAW ENDPOINTS (no validation) =====

# JSON body echo
app.post('/json/small', &echo_body)
app.post('/json/medium', &echo_body)
app.post('/json/large', &echo_body)
app.post('/json/very-large', &echo_body)

# Multipart
app.post('/multipart/small', &multipart_handler)
app.post('/multipart/medium', &multipart_handler)
app.post('/multipart/large', &multipart_handler)

# URL-encoded
app.post('/urlencoded/simple', &echo_body)
app.post('/urlencoded/complex', &echo_body)

# Path parameters
app.get('/path/simple/{id}') { |params, _query, _body| { 'id' => params['id'] || params[:id] } }
app.get('/path/multiple/{user_id}/{post_id}') do |params, _query, _body|
  { 'user_id' => params['user_id'] || params[:user_id],
    'post_id' => params['post_id'] || params[:post_id] }
end
app.get('/path/deep/{org}/{team}/{project}/{resource}/{id}') do |params, _query, _body|
  {
    'org' => params['org'] || params[:org],
    'team' => params['team'] || params[:team],
    'project' => params['project'] || params[:project],
    'resource' => params['resource'] || params[:resource],
    'id' => params['id'] || params[:id]
  }
end
app.get('/path/int/{id}') { |params, _query, _body| { 'id' => (params['id'] || params[:id]).to_i } }
app.get('/path/uuid/{uuid}') { |params, _query, _body| { 'uuid' => params['uuid'] || params[:uuid] } }
app.get('/path/date/{date}') { |params, _query, _body| { 'date' => params['date'] || params[:date] } }

# Query parameters
query_echo = lambda { |_params, query, _body| query || {} }
app.get('/query/few', &query_echo)
app.get('/query/medium', &query_echo)
app.get('/query/many', &query_echo)

# Health / root
app.get('/health') { |_params, _query, _body| { 'status' => 'ok' } }
app.get('/') { |_params, _query, _body| { 'status' => 'ok' } }

# ===== VALIDATED ENDPOINTS (with schemas) =====

# JSON body echo - validated
app.post('/validated/json/small',
         request_schema: request_schemas['json/small'],
         response_schema: response_schemas['json/small'], &echo_body)
app.post('/validated/json/medium',
         request_schema: request_schemas['json/medium'],
         response_schema: response_schemas['json/medium'], &echo_body)
app.post('/validated/json/large',
         request_schema: request_schemas['json/large'],
         response_schema: response_schemas['json/large'], &echo_body)
app.post('/validated/json/very-large',
         request_schema: request_schemas['json/very-large'],
         response_schema: response_schemas['json/very-large'], &echo_body)

# Multipart - validated
app.post('/validated/multipart/small',
         request_schema: request_schemas['multipart/small'],
         response_schema: response_schemas['multipart/small'], &multipart_handler)
app.post('/validated/multipart/medium',
         request_schema: request_schemas['multipart/medium'],
         response_schema: response_schemas['multipart/medium'], &multipart_handler)
app.post('/validated/multipart/large',
         request_schema: request_schemas['multipart/large'],
         response_schema: response_schemas['multipart/large'], &multipart_handler)

# URL-encoded - validated
app.post('/validated/urlencoded/simple',
         request_schema: request_schemas['urlencoded/simple'],
         response_schema: response_schemas['urlencoded/simple'], &echo_body)
app.post('/validated/urlencoded/complex',
         request_schema: request_schemas['urlencoded/complex'],
         response_schema: response_schemas['urlencoded/complex'], &echo_body)

# Path parameters - validated
app.get('/validated/path/simple/{id}',
        response_schema: response_schemas['path/simple'],
        parameter_schema: parameter_schemas['path/simple']) { |params, _query, _body| { 'id' => params['id'] || params[:id] } }
app.get('/validated/path/multiple/{user_id}/{post_id}',
        response_schema: response_schemas['path/multiple'],
        parameter_schema: parameter_schemas['path/multiple']) do |params, _query, _body|
  { 'user_id' => params['user_id'] || params[:user_id],
    'post_id' => params['post_id'] || params[:post_id] }
end
app.get('/validated/path/deep/{org}/{team}/{project}/{resource}/{id}',
        response_schema: response_schemas['path/deep'],
        parameter_schema: parameter_schemas['path/deep']) do |params, _query, _body|
  {
    'org' => params['org'] || params[:org],
    'team' => params['team'] || params[:team],
    'project' => params['project'] || params[:project],
    'resource' => params['resource'] || params[:resource],
    'id' => params['id'] || params[:id]
  }
end
app.get('/validated/path/int/{id}',
        response_schema: response_schemas['path/int'],
        parameter_schema: parameter_schemas['path/int']) { |params, _query, _body| { 'id' => (params['id'] || params[:id]).to_i } }
app.get('/validated/path/uuid/{uuid}',
        response_schema: response_schemas['path/uuid'],
        parameter_schema: parameter_schemas['path/uuid']) { |params, _query, _body| { 'uuid' => params['uuid'] || params[:uuid] } }
app.get('/validated/path/date/{date}',
        response_schema: response_schemas['path/date'],
        parameter_schema: parameter_schemas['path/date']) { |params, _query, _body| { 'date' => params['date'] || params[:date] } }

# Query parameters - validated
app.get('/validated/query/few',
        response_schema: response_schemas['query/few'],
        parameter_schema: parameter_schemas['query/few'], &query_echo)
app.get('/validated/query/medium',
        response_schema: response_schemas['query/medium'],
        parameter_schema: parameter_schemas['query/medium'], &query_echo)
app.get('/validated/query/many',
        response_schema: response_schemas['query/many'],
        parameter_schema: parameter_schemas['query/many'], &query_echo)

# Health / root - validated
app.get('/validated/health',
        response_schema: response_schemas['health']) { |_params, _query, _body| { 'status' => 'ok' } }
app.get('/validated/',
        response_schema: response_schemas['root']) { |_params, _query, _body| { 'status' => 'ok' } }

if __FILE__ == $0
  port = (ARGV[0] || 8000).to_i
  $stderr.puts "Starting Spikard-Ruby benchmark server on port #{port}"
  app.run(host: '0.0.0.0', port: port)
end
