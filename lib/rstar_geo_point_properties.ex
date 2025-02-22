defmodule RStarGeoPointProperties do
  use Rustler, otp_app: :rstar_geo_point_properties, crate: "rstar_geo_point_properties"

  def lookup(_lat, _lon), do: :erlang.nif_error(:nif_not_loaded)

  def init(_data), do: :erlang.nif_error(:nif_not_loaded)

  def init_local(_data), do: :erlang.nif_error(:nif_not_loaded)

  def lookup_local(_ref, _lat, _lon), do: :erlang.nif_error(:nif_not_loaded)
end
