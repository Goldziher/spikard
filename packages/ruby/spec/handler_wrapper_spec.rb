# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Spikard::HandlerWrapper do
  describe '.wrap_body_handler' do
    it 'wraps a block that receives only the body' do
      handler = described_class.wrap_body_handler do |body|
        { received: body }
      end

      expect(handler).to be_a(Proc)
    end

    it 'calls the handler with body when invoked' do
      handler = described_class.wrap_body_handler do |body|
        { filename: body[:file] }
      end

      result = handler.call({ ignored: 'params' }, { ignored: 'query' }, { file: 'test.txt' })
      expect(result).to eq({ filename: 'test.txt' })
    end

    it 'ignores path params passed to the wrapped proc' do
      handler = described_class.wrap_body_handler do |body|
        body
      end

      params = { id: '123' }
      query = { search: 'test' }
      body = { data: 'payload' }

      result = handler.call(params, query, body)
      expect(result).to eq(body)
    end

    it 'ignores query params passed to the wrapped proc' do
      handler = described_class.wrap_body_handler do |body|
        body.merge(processed: true)
      end

      query = { filter: 'active', sort: 'desc' }
      body = { items: [1, 2, 3] }

      result = handler.call({}, query, body)
      expect(result).to eq({ items: [1, 2, 3], processed: true })
    end

    it 'raises ArgumentError when no block is provided' do
      expect do
        described_class.wrap_body_handler
      end.to raise_error(ArgumentError, 'block required for wrap_body_handler')
    end

    it 'handles empty body' do
      handler = described_class.wrap_body_handler do |body|
        body.empty? ? { empty: true } : { empty: false }
      end

      result = handler.call({}, {}, {})
      expect(result).to eq({ empty: true })
    end

    it 'handles nil body' do
      handler = described_class.wrap_body_handler do |body|
        body.nil? ? { body_nil: true } : { body: body }
      end

      result = handler.call({}, {}, nil)
      expect(result).to eq({ body_nil: true })
    end

    it 'supports UploadFile objects in body' do
      upload_file = Spikard::UploadFile.new(
        'test.txt',
        'file content',
        content_type: 'text/plain'
      )

      handler = described_class.wrap_body_handler do |body|
        { filename: body[:file].filename, type: body[:file].content_type }
      end

      result = handler.call({}, {}, { file: upload_file })
      expect(result).to eq({ filename: 'test.txt', type: 'text/plain' })
    end

    it 'preserves complex nested structures in body' do
      handler = described_class.wrap_body_handler do |body|
        body
      end

      complex_body = {
        user: { id: 1, name: 'Alice' },
        items: [{ id: 1, qty: 5 }, { id: 2, qty: 3 }],
        metadata: { version: 1 }
      }

      result = handler.call({}, {}, complex_body)
      expect(result).to eq(complex_body)
    end

    it 'allows handler to transform the body' do
      handler = described_class.wrap_body_handler do |body|
        stringified = body.transform_keys(&:to_s)
        stringified.merge('processed_at' => Time.now.to_s)
      end

      body = { symbol_key: 'value' }
      result = handler.call({}, {}, body)

      expect(result['symbol_key']).to eq('value')
      expect(result).to have_key('processed_at')
    end
  end

  describe '.wrap_handler' do
    it 'wraps a block that receives params, query, and body' do
      handler = described_class.wrap_handler do |params, query, body|
        { params: params, query: query, body: body }
      end

      expect(handler).to be_a(Proc)
    end

    it 'passes all three parameters to the handler' do
      handler = described_class.wrap_handler do |params, query, body|
        {
          user_id: params[:id],
          search_query: query[:q],
          file_name: body[:file]
        }
      end

      result = handler.call(
        { id: '42' },
        { q: 'search term' },
        { file: 'document.pdf' }
      )

      expect(result).to eq(
        user_id: '42',
        search_query: 'search term',
        file_name: 'document.pdf'
      )
    end

    it 'handles empty params, query, and body' do
      handler = described_class.wrap_handler do |params, query, body|
        { params_empty: params.empty?, query_empty: query.empty?, body_empty: body.empty? }
      end

      result = handler.call({}, {}, {})
      expect(result).to eq(params_empty: true, query_empty: true, body_empty: true)
    end

    it 'raises ArgumentError when no block is provided' do
      expect do
        described_class.wrap_handler
      end.to raise_error(ArgumentError, 'block required for wrap_handler')
    end

    it 'preserves parameter values exactly as passed' do
      handler = described_class.wrap_handler do |params, query, body|
        [params, query, body]
      end

      params = { user_id: 123, role: 'admin' }
      query = { limit: 10, offset: 0 }
      body = { name: 'John', tags: %w[a b] }

      result = handler.call(params, query, body)

      expect(result[0]).to equal(params)
      expect(result[1]).to equal(query)
      expect(result[2]).to equal(body)
    end

    it 'allows handler to access nested parameters' do
      handler = described_class.wrap_handler do |params, query, body|
        {
          org_id: params.dig(:org, :id),
          filter_active: query.dig(:filter, :active),
          nested_data: body.dig(:data, :items, 0)
        }
      end

      result = handler.call(
        { org: { id: 'org-123' } },
        { filter: { active: true } },
        { data: { items: ['first'] } }
      )

      expect(result[:org_id]).to eq('org-123')
      expect(result[:filter_active]).to be true
      expect(result[:nested_data]).to eq('first')
    end

    it 'returns various data types from handler' do
      handler = described_class.wrap_handler do |params, _query, _body|
        params[:return_type] == 'hash' ? { status: 'ok' } : 'plain text response'
      end

      hash_result = handler.call({ return_type: 'hash' }, {}, {})
      expect(hash_result).to be_a(Hash)
      expect(hash_result).to eq(status: 'ok')

      string_result = handler.call({ return_type: 'string' }, {}, {})
      expect(string_result).to be_a(String)
      expect(string_result).to eq('plain text response')
    end

    it 'handles complex return values' do
      handler = described_class.wrap_handler do |params, query, _body|
        {
          user_id: params[:id],
          created: true,
          items: query[:items] || []
        }
      end

      result = handler.call({ id: '999' }, { items: %w[a b] }, {})
      expect(result).to be_a(Hash)
      expect(result[:user_id]).to eq('999')
      expect(result[:created]).to be true
      expect(result[:items]).to eq(%w[a b])
    end
  end

  describe '.wrap_handler_with_context' do
    it 'wraps a block that receives a context hash' do
      handler = described_class.wrap_handler_with_context do |ctx|
        { context: ctx }
      end

      expect(handler).to be_a(Proc)
    end

    it 'bundles params, query, and body into a context hash' do
      handler = described_class.wrap_handler_with_context do |ctx|
        {
          params_present: ctx.key?(:params),
          query_present: ctx.key?(:query),
          body_present: ctx.key?(:body)
        }
      end

      result = handler.call({ id: '1' }, { q: 'test' }, { data: 'payload' })
      expect(result).to eq(params_present: true, query_present: true, body_present: true)
    end

    it 'provides correct values in the context hash' do
      handler = described_class.wrap_handler_with_context do |ctx|
        {
          user_id: ctx[:params][:id],
          search: ctx[:query][:q],
          file: ctx[:body][:file]
        }
      end

      result = handler.call(
        { id: '42' },
        { q: 'needle' },
        { file: 'image.png' }
      )

      expect(result).to eq(user_id: '42', search: 'needle', file: 'image.png')
    end

    it 'raises ArgumentError when no block is provided' do
      expect do
        described_class.wrap_handler_with_context
      end.to raise_error(ArgumentError, 'block required for wrap_handler_with_context')
    end

    it 'handles empty context values gracefully' do
      handler = described_class.wrap_handler_with_context do |ctx|
        {
          params: ctx[:params],
          query: ctx[:query],
          body: ctx[:body]
        }
      end

      result = handler.call({}, {}, {})
      expect(result).to eq(params: {}, query: {}, body: {})
    end

    it 'preserves context structure for nested access' do
      handler = described_class.wrap_handler_with_context do |ctx|
        {
          nested_param: ctx[:params].dig(:user, :id),
          nested_query: ctx[:query].dig(:filter, :status),
          nested_body: ctx[:body].dig(:data, :items)
        }
      end

      result = handler.call(
        { user: { id: 'u-123' } },
        { filter: { status: 'active' } },
        { data: { items: [1, 2, 3] } }
      )

      expect(result[:nested_param]).to eq('u-123')
      expect(result[:nested_query]).to eq('active')
      expect(result[:nested_body]).to eq([1, 2, 3])
    end

    it 'returns various response types from context handler' do
      handler = described_class.wrap_handler_with_context do |ctx|
        {
          request_info: {
            has_params: !ctx[:params].empty?,
            has_query: !ctx[:query].empty?,
            has_body: !ctx[:body].empty?
          }
        }
      end

      result = handler.call({ key: 'value' }, { search: 'term' }, { data: 'content' })
      expect(result).to be_a(Hash)
      expect(result[:request_info][:has_params]).to be true
      expect(result[:request_info][:has_query]).to be true
      expect(result[:request_info][:has_body]).to be true
    end

    it 'allows modification of context within handler' do
      handler = described_class.wrap_handler_with_context do |ctx|
        ctx[:params] = ctx[:params].merge(processed: true)
        ctx
      end

      result = handler.call({ id: '1' }, {}, {})
      expect(result[:params]).to include(processed: true)
    end
  end

  describe 'handler composition' do
    it 'wraps multiple handlers independently' do
      handler1 = described_class.wrap_body_handler { |body| { h1: body } }
      handler2 = described_class.wrap_handler { |_p, _q, b| { h2: b } }
      handler3 = described_class.wrap_handler_with_context { |ctx| { h3: ctx } }

      result1 = handler1.call({}, {}, { data: 'test' })
      result2 = handler2.call({}, {}, { data: 'test' })
      result3 = handler3.call({}, {}, { data: 'test' })

      expect(result1).to have_key(:h1)
      expect(result2).to have_key(:h2)
      expect(result3).to have_key(:h3)
    end

    it 'wraps handlers with shared business logic' do
      shared_transform = ->(data) { data.transform_keys(&:to_s) }

      handler1 = described_class.wrap_body_handler do |body|
        shared_transform.call(body)
      end

      handler2 = described_class.wrap_handler do |_params, _query, body|
        shared_transform.call(body)
      end

      result1 = handler1.call({}, {}, { key: 'value' })
      result2 = handler2.call({}, {}, { key: 'value' })

      expect(result1).to eq('key' => 'value')
      expect(result2).to eq('key' => 'value')
    end
  end

  describe 'error handling' do
    it 'propagates exceptions raised in body handler' do
      handler = described_class.wrap_body_handler do |_body|
        raise StandardError, 'handler error'
      end

      expect do
        handler.call({}, {}, {})
      end.to raise_error(StandardError, 'handler error')
    end

    it 'propagates exceptions raised in full handler' do
      handler = described_class.wrap_handler do |_params, _query, _body|
        raise 'processing failed'
      end

      expect do
        handler.call({}, {}, {})
      end.to raise_error(RuntimeError, 'processing failed')
    end

    it 'propagates exceptions raised in context handler' do
      handler = described_class.wrap_handler_with_context do |_ctx|
        raise ArgumentError, 'invalid context'
      end

      expect do
        handler.call({}, {}, {})
      end.to raise_error(ArgumentError, 'invalid context')
    end

    it 'handles NoMethodError gracefully by re-raising' do
      handler = described_class.wrap_body_handler(&:undefined_method)

      expect do
        handler.call({}, {}, {})
      end.to raise_error(NoMethodError)
    end

    it 'preserves error backtrace information' do
      handler = described_class.wrap_handler do |_params, _query, _body|
        raise 'test error'
      end

      begin
        handler.call({}, {}, {})
      rescue StandardError => e
        expect(e.backtrace).not_to be_empty
      end
    end
  end

  describe 'edge cases' do
    it 'handles symbols as parameter keys' do
      handler = described_class.wrap_handler do |params, _query, _body|
        { param_value: params[:user_id] }
      end

      result = handler.call({ user_id: '999' }, {}, {})
      expect(result).to eq(param_value: '999')
    end

    it 'handles string keys as parameter keys' do
      handler = described_class.wrap_handler do |params, _query, _body|
        { param_value: params['user_id'] }
      end

      result = handler.call({ 'user_id' => '999' }, {}, {})
      expect(result).to eq(param_value: '999')
    end

    it 'handles mixed symbol and string keys' do
      handler = described_class.wrap_body_handler do |body|
        { sym: body[:symbol_key], str: body['string_key'] }
      end

      result = handler.call({}, {}, { symbol_key: 'sym_val', 'string_key' => 'str_val' })
      expect(result).to eq(sym: 'sym_val', str: 'str_val')
    end

    it 'handles numeric values in parameters' do
      handler = described_class.wrap_handler do |params, query, body|
        { int: params[:count], float: query[:price], bool: body[:active] }
      end

      result = handler.call(
        { count: 42 },
        { price: 19.99 },
        { active: true }
      )

      expect(result).to eq(int: 42, float: 19.99, bool: true)
    end

    it 'handles array values in parameters' do
      handler = described_class.wrap_handler do |params, query, _body|
        { ids: params[:ids], tags: query[:tags] }
      end

      result = handler.call(
        { ids: [1, 2, 3] },
        { tags: %w[ruby rails rspec] },
        {}
      )

      expect(result).to eq(ids: [1, 2, 3], tags: %w[ruby rails rspec])
    end

    it 'handles null/nil values in parameters' do
      handler = described_class.wrap_handler do |params, query, body|
        { p: params[:val], q: query[:val], b: body[:val] }
      end

      result = handler.call({ val: nil }, { val: nil }, { val: nil })
      expect(result).to eq(p: nil, q: nil, b: nil)
    end

    it 'works with Proc objects directly' do
      my_proc = proc { |body| { proc_result: body } }
      handler = described_class.wrap_body_handler(&my_proc)

      result = handler.call({}, {}, { data: 'from proc' })
      expect(result).to eq(proc_result: { data: 'from proc' })
    end

    it 'works with Lambda objects directly' do
      my_lambda = ->(body) { { lambda_result: body } }
      handler = described_class.wrap_body_handler(&my_lambda)

      result = handler.call({}, {}, { data: 'from lambda' })
      expect(result).to eq(lambda_result: { data: 'from lambda' })
    end
  end
end
