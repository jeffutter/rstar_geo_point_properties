defmodule RStarGeoPointProperties.MixProject do
  use Mix.Project

  @version "0.1.0"
  @source_url "https://github.com/jeffutter/rstar_geo_point_properties"

  def project do
    [
      app: :rstar_geo_point_properties,
      version: @version,
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      package: package(),
      description: description(),
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler_precompiled, "~> 0.8"},
      {:rustler, "~> 0.36.1"},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
    ]
  end

  defp description() do
    "A small package for finding properties for features of a given point in a GeoJSON file."
  end

  defp package do
    [
      files: [
        "lib",
        "native",
        "checksum-*.exs",
        "mix.exs"
      ],
      licenses: ["MIT"],
      links: %{"GitHub" => @source_url}
    ]
  end
end
