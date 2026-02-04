defmodule Spikard.Native do
  @moduledoc false
  # NIF stub module - functions are loaded from Rust via Rustler

  use Rustler,
    otp_app: :spikard,
    crate: "spikard_elixir",
    path: "../../crates/spikard-elixir",
    mode: if(System.get_env("SPIKARD_BUILD") in ["1", "true"], do: :release, else: :release)

  # When NIFs are not loaded, these functions return an error
  def nif_not_loaded, do: :erlang.nif_error(:nif_not_loaded)
end
