# frozen_string_literal: true

require 'spec_helper'

RSpec.describe "Spikard Ruby binding" do
  it "returns the embedded version" do
    expect(Spikard.version).to eq(Spikard::VERSION)
  end
end
