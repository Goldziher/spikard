# frozen_string_literal: true

require 'pathname'
require 'json'

RSpec.describe 'DI doc snippet' do
  it 'evaluates and registers dependencies' do
    begin
      require 'spikard'
    rescue LoadError
      skip 'spikard native extension not built; skipping doc snippet eval'
    end

    snippet_path = Pathname.new(__dir__).join('..', '..', '..', 'docs', 'snippets', 'ruby',
                                              'dependency_injection.md').cleanpath
    content = snippet_path.read
    snippet = content.match(/```ruby\s*(?<code>.*?)```/m)
    raise 'DI snippet not found' unless snippet

    code = snippet[:code]
    app = nil
    eval(code, binding) # rubocop:disable Security/Eval

    expect(defined?(app)).to eq('local-variable')
    deps = app.dependencies
    expect(deps.keys).to include('db_pool', 'config')
  end
end
