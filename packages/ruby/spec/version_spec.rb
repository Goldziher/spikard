# frozen_string_literal: true

RSpec.describe Spikard do
  it 'exposes a version constant' do
    expect(Spikard::VERSION).to be_a(String)
  end

  it 'exposes the native extension version' do
    expect(Spikard.version).to eq(Spikard::VERSION)
  end
end
