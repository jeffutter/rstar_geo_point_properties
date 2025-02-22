defmodule RStarGeoPointPropertiesTest do
  use ExUnit.Case
  doctest RStarGeoPointProperties

  @kalamazoo %{lat: 42.29171, lon: -85.58723}

  test "returns a parse error on invalid geojson" do
    assert {:error, :parse_error} = RStarGeoPointProperties.init_local("")
  end

  # Note, this is the only test that can use `init/1` as it loads the data globally
  # Other tests should use `init_local/1`
  test "Finds Properties" do
    assert {:error, :not_loaded} = RStarGeoPointProperties.lookup(@kalamazoo.lat, @kalamazoo.lon)
    :ok = RStarGeoPointProperties.init(data())
    properties = RStarGeoPointProperties.lookup(@kalamazoo.lat, @kalamazoo.lon)

    assert {:ok, [%{"name" => "Michigan"}]} = properties

    {:error, :already_loaded} = RStarGeoPointProperties.init(data())
  end

  test "Finds Properties Local" do
    {:ok, tree} = RStarGeoPointProperties.init_local(data())
    properties = RStarGeoPointProperties.lookup_local(tree, @kalamazoo.lat, @kalamazoo.lon)

    assert {:ok, [%{"name" => "Michigan"}]} = properties
  end

  def data do
    File.read!("./test/fixtures/us-states.json")
  end
end
