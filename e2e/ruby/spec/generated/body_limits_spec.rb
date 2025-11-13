# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "body_limits" do
  it "Body over limit returns 413" do
    app = E2ERubyApp.create_app_body_limits_1_body_over_limit_returns_413
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/body-limit/over", json: {"note" => "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"})
    expect(response.status_code).to eq(413)
    client.close
  end

  it "Body under limit succeeds" do
    app = E2ERubyApp.create_app_body_limits_2_body_under_limit_succeeds
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/body-limit/under", json: {"note" => "small"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"accepted" => true, "note" => "small"})
    client.close
  end

end
