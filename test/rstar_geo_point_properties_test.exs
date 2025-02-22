defmodule RStarGeoPointPropertiesTest do
  use ExUnit.Case
  doctest RStarGeoPointProperties

  @kalamazoo %{lat: 42.29171, lon: -85.58723}

  test "Finds Properties" do
    assert {:error, :not_loaded} = RStarGeoPointProperties.lookup(@kalamazoo.lat, @kalamazoo.lon)
    assert {:error, :parse_error} = RStarGeoPointProperties.init("")

    data = File.read!("./test/fixtures/us-states.json")

    :ok = RStarGeoPointProperties.init(data)
    properties = RStarGeoPointProperties.lookup(@kalamazoo.lat, @kalamazoo.lon)

    assert {:ok, [%{"name" => "Michigan"}]} = properties
  end

  test "Finds Properties Local" do
    data = File.read!("./test/fixtures/us-states.json")

    {:ok, tree} = RStarGeoPointProperties.init_local(data)
    properties = RStarGeoPointProperties.lookup_local(tree, @kalamazoo.lat, @kalamazoo.lon)

    assert {:ok, [%{"name" => "Michigan"}]} = properties
  end
end
