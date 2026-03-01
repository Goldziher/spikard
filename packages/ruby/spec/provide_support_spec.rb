# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Spikard::ProvideSupport do
  # Test helper registry that captures dependency registrations.
  class FakeRegistry # :nodoc:
    attr_reader :factories, :values

    def initialize
      @factories = []
      @values = []
    end

    def register_factory(key, factory, depends_on, singleton, cacheable)
      @factories << [key, factory, depends_on, singleton, cacheable]
    end

    def register_value(key, value)
      @values << [key, value]
    end
  end

  let(:registry) { FakeRegistry.new }
  let(:container_class) do
    Class.new do
      include Spikard::ProvideSupport

      attr_accessor :native_dependencies
    end
  end
  let(:container) do
    instance = container_class.new
    instance.native_dependencies = registry
    instance
  end

  it 'registers Provide wrappers as factories' do
    provider = Spikard::Provide.new(proc {}, depends_on: %w[db], singleton: true, cacheable: false)

    container.provide('svc', provider)

    expect(registry.factories.last).to match(['svc', provider.factory, ['db'], true, false])
  end

  it 'registers blocks as factories with dependency coercion' do
    block = proc { :value }

    container.provide('svc', depends_on: [:foo], singleton: true, cacheable: false, &block)

    expect(registry.factories.last).to match(['svc', block, ['foo'], true, false])
  end

  it 'registers values when provided without blocks' do
    container.provide('config', { 'k' => 'v' })

    expect(registry.values.last).to eq(['config', { 'k' => 'v' }])
  end

  it 'raises when neither value nor block is provided' do
    expect { container.provide('empty') }.to raise_error(ArgumentError)
  end

  it 'returns dependencies via the native registry' do
    expect(container.dependencies).to eq(registry)
  end

  it 'raises when native registry is missing' do
    missing = container_class.new

    expect { missing.send(:ensure_native_dependencies!) }.to raise_error(RuntimeError)
  end

  it 'wraps handlers with dependency injection' do
    deps = {
      'db' => { type: :value, value: 2 }
    }
    handler = ->(request, db:) { [request, db * 3] }

    wrapped = Spikard::DIHandlerWrapper.wrap_handler(handler, deps)
    result_request, value = wrapped.call(:req)

    expect(result_request).to eq(:req)
    expect(value).to eq(6)
  end

  it 'resolves factory dependencies via DI handler' do
    calls = 0
    factory = lambda {
      calls += 1
      :made
    }
    dep_def = { type: :factory, factory: factory, depends_on: [] }

    expect(Spikard::DIHandlerWrapper.resolve_dependency(dep_def, nil)).to eq(:made)
    expect(calls).to eq(1)
  end

  it 'resolves nested factory dependencies with keyword arguments' do
    deps = {
      'config' => { type: :value, value: { 'db_url' => 'postgresql://localhost/app' } },
      'db_pool' => {
        type: :factory,
        depends_on: ['config'],
        factory: ->(config:) { "pool:#{config['db_url']}" }
      }
    }
    handler = ->(db_pool:) { db_pool }

    wrapped = Spikard::DIHandlerWrapper.wrap_handler(handler, deps)

    expect(wrapped.call(:request)).to eq('pool:postgresql://localhost/app')
  end

  it 'resolves nested factory dependencies with positional arguments' do
    deps = {
      'db_url' => { type: :value, value: 'postgresql://localhost/app' },
      'db_pool' => {
        type: :factory,
        depends_on: ['db_url'],
        factory: ->(db_url) { "pool:#{db_url}" }
      }
    }

    expect(
      Spikard::DIHandlerWrapper.resolve_dependency_by_key('db_pool', deps, nil, {}, {}, [])
    ).to eq('pool:postgresql://localhost/app')
  end

  it 'caches singleton factory dependencies across calls' do
    calls = 0
    deps = {
      'counter' => {
        type: :factory,
        singleton: true,
        depends_on: [],
        factory: lambda {
          calls += 1
          calls
        }
      }
    }
    handler = ->(counter:) { counter }

    wrapped = Spikard::DIHandlerWrapper.wrap_handler(handler, deps)

    expect(wrapped.call(:request)).to eq(1)
    expect(wrapped.call(:request)).to eq(1)
    expect(calls).to eq(1)
  end

  it 'raises on missing nested dependencies' do
    dep_def = {
      type: :factory,
      depends_on: ['config'],
      factory: ->(config:) { config }
    }

    expect do
      Spikard::DIHandlerWrapper.resolve_dependency(dep_def, nil, {})
    end.to raise_error(KeyError, /Missing dependency: config/)
  end

  it 'raises on circular dependencies' do
    deps = {
      'a' => {
        type: :factory,
        depends_on: ['b'],
        factory: ->(b:) { b }
      },
      'b' => {
        type: :factory,
        depends_on: ['a'],
        factory: ->(a:) { a }
      }
    }

    expect do
      Spikard::DIHandlerWrapper.resolve_dependency_by_key('a', deps, nil, {}, {}, [])
    end.to raise_error(ArgumentError, /Circular dependency detected: a -> b -> a/)
  end

  it 'marks Provide helpers as sync primitives' do
    provider = Spikard::Provide.new(proc {})

    expect(provider.async?).to be(false)
    expect(provider.async_generator?).to be(false)
  end
end
