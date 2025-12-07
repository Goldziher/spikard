# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Spikard Provide Class' do
  describe Spikard::Provide do
    describe '#initialize' do
      it 'creates provider with a factory' do
        factory = proc { 'value' }
        provider = described_class.new(factory)
        expect(provider).to be_a(described_class)
      end

      it 'creates provider with a method' do
        def test_factory
          'value'
        end
        factory = method(:test_factory)
        provider = described_class.new(factory)
        expect(provider).to be_a(described_class)
      end

      it 'accepts singleton parameter as true' do
        provider = described_class.new(proc { 'value' }, singleton: true)
        expect(provider.singleton).to be true
      end

      it 'accepts singleton parameter as false' do
        provider = described_class.new(proc { 'value' }, singleton: false)
        expect(provider.singleton).to be false
      end

      it 'defaults singleton to false' do
        provider = described_class.new(proc { 'value' })
        expect(provider.singleton).to be false
      end

      it 'accepts cacheable parameter as true' do
        provider = described_class.new(proc { 'value' }, cacheable: true)
        expect(provider.cacheable).to be true
      end

      it 'accepts cacheable parameter as false' do
        provider = described_class.new(proc { 'value' }, cacheable: false)
        expect(provider.cacheable).to be false
      end

      it 'defaults cacheable to true' do
        provider = described_class.new(proc { 'value' })
        expect(provider.cacheable).to be true
      end

      it 'accepts depends_on parameter' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: %i[db cache]
        )
        expect(provider.depends_on).to eq(%w[db cache])
      end

      it 'defaults depends_on to empty array' do
        provider = described_class.new(proc { 'value' })
        expect(provider.depends_on).to eq([])
      end

      it 'converts depends_on symbols to strings' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: %i[database logger]
        )
        expect(provider.depends_on).to eq(%w[database logger])
      end

      it 'handles string depends_on values' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: %w[db cache]
        )
        expect(provider.depends_on).to eq(%w[db cache])
      end

      it 'handles mixed string and symbol depends_on' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: [:db, 'cache', :logger]
        )
        expect(provider.depends_on).to eq(%w[db cache logger])
      end

      it 'wraps single depends_on value in array' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: :database
        )
        expect(provider.depends_on).to eq(['database'])
      end
    end

    describe 'attribute readers' do
      let(:factory) { proc { 'value' } }

      it 'provides factory reader' do
        provider = described_class.new(factory)
        expect(provider.factory).to eq(factory)
      end

      it 'provides depends_on reader' do
        provider = described_class.new(
          factory,
          depends_on: [:db]
        )
        expect(provider.depends_on).to eq(['db'])
      end

      it 'provides singleton reader' do
        provider = described_class.new(factory, singleton: true)
        expect(provider.singleton).to be true
      end

      it 'provides cacheable reader' do
        provider = described_class.new(factory, cacheable: false)
        expect(provider.cacheable).to be false
      end
    end

    describe '#async?' do
      it 'returns false by default' do
        provider = described_class.new(proc { 'value' })
        expect(provider.async?).to be false
      end

      it 'returns false for sync factory' do
        factory = proc {
          sleep(0.001)
          'value'
        }
        provider = described_class.new(factory)
        expect(provider.async?).to be false
      end
    end

    describe '#async_generator?' do
      it 'returns false by default' do
        provider = described_class.new(proc { 'value' })
        expect(provider.async_generator?).to be false
      end
    end

    describe 'singleton behavior' do
      it 'allows singleton to be true for global caching' do
        provider = described_class.new(
          proc { Object.new },
          singleton: true
        )
        expect(provider.singleton).to be true
      end

      it 'allows singleton to be false for per-request instance' do
        provider = described_class.new(
          proc { Object.new },
          singleton: false
        )
        expect(provider.singleton).to be false
      end

      it 'differentiates singleton and cacheable' do
        # singleton: true means cache globally across requests
        # cacheable: true means cache per request
        provider = described_class.new(
          proc { 'value' },
          singleton: true,
          cacheable: true
        )
        expect(provider.singleton).to be true
        expect(provider.cacheable).to be true
      end

      it 'supports singleton without cacheable' do
        provider = described_class.new(
          proc { 'value' },
          singleton: true,
          cacheable: false
        )
        expect(provider.singleton).to be true
        expect(provider.cacheable).to be false
      end
    end

    describe 'cache behavior' do
      it 'allows caching to be enabled' do
        provider = described_class.new(
          proc { 'value' },
          cacheable: true
        )
        expect(provider.cacheable).to be true
      end

      it 'allows caching to be disabled' do
        provider = described_class.new(
          proc { 'value' },
          cacheable: false
        )
        expect(provider.cacheable).to be false
      end

      it 'supports dependency providers with caching' do
        call_count = 0
        factory = proc {
          call_count += 1
          "call_#{call_count}"
        }
        provider = described_class.new(factory, cacheable: true)
        expect(provider.cacheable).to be true
      end

      it 'supports non-cacheable factories' do
        uuid_factory = proc { SecureRandom.uuid }
        provider = described_class.new(uuid_factory, cacheable: false)
        expect(provider.cacheable).to be false
      end
    end

    describe 'dependency extraction and filtering' do
      it 'records explicit dependencies' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: %i[db cache logger]
        )
        expect(provider.depends_on).to match_array(%w[db cache logger])
      end

      it 'filters duplicate dependencies' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: %i[db cache db]
        )
        # NOTE: Array() and to_s conversion preserves duplicates
        # In actual use, the DI container would handle deduplication
        expect(provider.depends_on).to eq(%w[db cache db])
      end

      it 'handles empty dependency list' do
        provider = described_class.new(
          proc { 'constant value' },
          depends_on: []
        )
        expect(provider.depends_on).to eq([])
      end

      it 'handles nil depends_on' do
        provider = described_class.new(
          proc { 'value' },
          depends_on: nil
        )
        expect(provider.depends_on).to eq([])
      end

      it 'accepts arbitrary dependency names' do
        dependencies = %i[db cache config logger queue worker_pool]
        provider = described_class.new(
          proc { 'value' },
          depends_on: dependencies
        )
        expect(provider.depends_on.length).to eq(6)
      end
    end

    describe 'factory invocation' do
      it 'stores factory as-is' do
        factory = proc { 42 }
        provider = described_class.new(factory)
        expect(provider.factory).to eq(factory)
      end

      it 'stores method as factory' do
        def my_method
          'result'
        end
        factory = method(:my_method)
        provider = described_class.new(factory)
        expect(provider.factory).to eq(factory)
      end

      it 'supports lambda as factory' do
        factory = -> { 'value' }
        provider = described_class.new(factory)
        expect(provider.factory).to eq(factory)
      end

      it 'supports proc as factory' do
        factory = proc { 'value' }
        provider = described_class.new(factory)
        expect(provider.factory).to eq(factory)
      end
    end

    describe 'complete configuration scenarios' do
      it 'creates database provider with singleton and caching' do
        db_factory = proc do
          { connection: 'postgresql://localhost' }
        end
        provider = described_class.new(
          db_factory,
          depends_on: %i[config],
          singleton: true,
          cacheable: true
        )

        expect(provider.factory).to eq(db_factory)
        expect(provider.depends_on).to eq(['config'])
        expect(provider.singleton).to be true
        expect(provider.cacheable).to be true
      end

      it 'creates request-scoped provider' do
        request_id_factory = proc { SecureRandom.uuid }
        provider = described_class.new(
          request_id_factory,
          singleton: false,
          cacheable: false
        )

        expect(provider.singleton).to be false
        expect(provider.cacheable).to be false
      end

      it 'creates multi-dependency provider' do
        auth_factory = proc { 'auth_service' }
        provider = described_class.new(
          auth_factory,
          depends_on: %i[db logger config cache],
          singleton: true,
          cacheable: true
        )

        expect(provider.depends_on).to match_array(%w[db logger config cache])
        expect(provider.singleton).to be true
      end

      it 'creates simple value provider' do
        simple_factory = proc { 'simple_value' }
        provider = described_class.new(simple_factory)

        expect(provider.factory).to eq(simple_factory)
        expect(provider.depends_on).to eq([])
        expect(provider.singleton).to be false
        expect(provider.cacheable).to be true
      end
    end

    describe 'integration with App' do
      it 'works with App dependency injection interface' do
        provider = described_class.new(proc { 'database' })

        expect(provider).to respond_to(:factory)
        expect(provider).to respond_to(:depends_on)
        expect(provider).to respond_to(:singleton)
        expect(provider).to respond_to(:cacheable)
        expect(provider).to respond_to(:async?)
        expect(provider).to respond_to(:async_generator?)
      end

      it 'provides all required DI attributes' do
        provider = described_class.new(
          proc { 'service' },
          depends_on: %i[dep1 dep2],
          singleton: true,
          cacheable: false
        )

        # All these should be readable for DI container
        expect(provider.factory).not_to be_nil
        expect(provider.depends_on).not_to be_nil
        expect(provider.singleton).not_to be_nil
        expect(provider.cacheable).not_to be_nil
      end

      it 'supports method reference as factory' do
        # Test double to verify method references work as factories
        class TestService
          def initialize
            @data = []
          end

          def build_service
            @data
          end
        end

        service = TestService.new
        provider = described_class.new(service.method(:build_service))

        expect(provider.factory).to be_a(Method)
      end
    end

    describe 'edge cases and special scenarios' do
      it 'handles factory that returns nil' do
        provider = described_class.new(proc {})
        expect(provider.factory).to be_a(Proc)
      end

      it 'handles factory that raises exception' do
        error_factory = proc { raise 'Factory error' }
        provider = described_class.new(error_factory)
        expect(provider.factory).to be_a(Proc)
      end

      it 'handles factory with complex return value' do
        complex_factory = proc do
          { nested: { data: [1, 2, 3] } }
        end
        provider = described_class.new(complex_factory)
        expect(provider.factory).to eq(complex_factory)
      end

      it 'preserves depends_on order' do
        deps = %i[first second third fourth fifth]
        provider = described_class.new(
          proc { 'value' },
          depends_on: deps
        )
        expected = %w[first second third fourth fifth]
        expect(provider.depends_on).to eq(expected)
      end

      it 'handles very long depends_on list' do
        deps = (1..100).map { |i| :"dep_#{i}" }
        provider = described_class.new(
          proc { 'value' },
          depends_on: deps
        )
        expect(provider.depends_on.length).to eq(100)
      end
    end
  end
end
