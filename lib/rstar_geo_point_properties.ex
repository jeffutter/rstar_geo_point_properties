defmodule RStarGeoPointProperties do
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :rstar_geo_point_properties,
    crate: "rstar_geo_point_properties",
    base_url:
      "https://github.com/jeffutter/rstar_geo_point_properties/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_BUILD") in ["1", "true"],
    targets: [
      "aarch64-apple-darwin",
      "x86_64-unknown-linux-gnu",
      "x86_64-unknown-linux-musl"
    ],
    version: version

  def lookup(_lat, _lon), do: :erlang.nif_error(:nif_not_loaded)

  def init(_data), do: :erlang.nif_error(:nif_not_loaded)

  def init_local(_data), do: :erlang.nif_error(:nif_not_loaded)

  def lookup_local(_ref, _lat, _lon), do: :erlang.nif_error(:nif_not_loaded)
end
