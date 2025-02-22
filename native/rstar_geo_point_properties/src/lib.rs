use geo::{BoundingRect, Contains, CoordsIter, Geometry, Point, Polygon};
use geojson::{Feature, FeatureCollection, GeoJson};
use rstar::{primitives::CachedEnvelope, PointDistance, RTree, RTreeObject, AABB};
use rustler::{Env, NifResult, ResourceArc, Term};
use std::{iter, sync::OnceLock};

static DATA: OnceLock<RTree<CachedEnvelope<FeatureGeom>>> = OnceLock::new();

mod my_atoms {
    rustler::atoms! {
        not_loaded,
        unknown,
        parse_error
    }
}

#[derive(Debug, Clone)]
struct FeatureGeom {
    geom: Polygon<f64>,
    feature: Feature,
}

impl FeatureGeom {
    pub fn new(geom: Polygon<f64>, feature: Feature) -> Self {
        Self { geom, feature }
    }
}

impl RTreeObject for FeatureGeom {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let bbox = self.geom.bounding_rect().unwrap();
        AABB::from_corners([bbox.min().x, bbox.min().y], [bbox.max().x, bbox.max().y])
    }
}

impl PointDistance for FeatureGeom {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let query_point = Point::new(point[0], point[1]);

        // If the point is inside the polygon, return distance 0
        if self.geom.contains(&query_point) {
            return 0.0;
        }

        // Otherwise, find the squared distance to the closest boundary point
        self.geom
            .exterior()
            .coords_iter()
            .map(|c| (c.x - point[0]).powi(2) + (c.y - point[1]).powi(2))
            .fold(f64::INFINITY, f64::min)
    }
}

fn feature_collection_to_iter(
    feature_collection: FeatureCollection,
) -> impl Iterator<Item = FeatureGeom> {
    feature_collection
        .into_iter()
        .filter_map(|f| {
            f.geometry
                .clone()
                .and_then(|g| geo::Geometry::<f64>::try_from(g).map(|g| (g, f)).ok())
        })
        .flat_map(geometry_to_iter)
}

fn geometry_to_iter(
    (geometry, feature): (geo::Geometry, Feature),
) -> Box<dyn Iterator<Item = FeatureGeom>> {
    match geometry {
        Geometry::Polygon(polygon) => Box::new(iter::once(FeatureGeom::new(polygon, feature))),
        Geometry::MultiPolygon(multi_polygon) => Box::new(
            multi_polygon
                .into_iter()
                .map(move |p| FeatureGeom::new(p, feature.clone())),
        ),
        Geometry::GeometryCollection(geometry_collection) => Box::new(
            geometry_collection
                .into_iter()
                .flat_map(move |g| geometry_to_iter((g, feature.clone()))),
        ),
        Geometry::Rect(rect) => Box::new(iter::once(FeatureGeom::new(rect.to_polygon(), feature))),
        Geometry::Triangle(triangle) => {
            Box::new(iter::once(FeatureGeom::new(triangle.to_polygon(), feature)))
        }
        _ => unimplemented!(),
    }
}

pub struct Geo(RTree<CachedEnvelope<FeatureGeom>>);
impl rustler::Resource for Geo {}

pub fn on_load(env: Env, _info: Term) -> bool {
    env.register::<Geo>().is_ok()
}

#[rustler::nif]
fn init(data: String) -> NifResult<rustler::Atom> {
    match data.parse::<GeoJson>() {
        Ok(data) => {
            if let GeoJson::FeatureCollection(feature_collection) = data {
                let features = feature_collection_to_iter(feature_collection)
                    .map(CachedEnvelope::new)
                    .collect::<Vec<_>>();

                let rtree = RTree::bulk_load(features);

                DATA.get_or_init(|| rtree);

                Ok(rustler::types::atom::ok())
            } else {
                unimplemented!()
            }
        }
        Err(_) => Err(rustler::Error::Term(Box::new(my_atoms::parse_error()))),
    }
}

#[rustler::nif]
fn init_local(data: String) -> Result<ResourceArc<Geo>, rustler::Atom> {
    let data: GeoJson = data.parse().map_err(|_| my_atoms::parse_error())?;

    if let GeoJson::FeatureCollection(feature_collection) = data {
        let features = feature_collection_to_iter(feature_collection)
            .map(CachedEnvelope::new)
            .collect::<Vec<_>>();

        let rtree = RTree::bulk_load(features);

        Ok(ResourceArc::new(Geo(rtree)))
    } else {
        unimplemented!()
    }
}

#[rustler::nif]
fn lookup(env: Env<'_>, lat: f64, lon: f64) -> Result<Vec<Term<'_>>, rustler::Atom> {
    match DATA.get() {
        Some(data) => _lookup(env, data, lat, lon).map_err(|_| my_atoms::unknown()),
        None => Err(my_atoms::not_loaded()),
    }
}

#[rustler::nif]
fn lookup_local(
    env: Env<'_>,
    geo: ResourceArc<Geo>,
    lat: f64,
    lon: f64,
) -> Result<Vec<Term<'_>>, rustler::Atom> {
    _lookup(env, &geo.0, lat, lon).map_err(|_| my_atoms::unknown())
}

fn _lookup<'a>(
    env: Env<'a>,
    tree: &RTree<CachedEnvelope<FeatureGeom>>,
    lat: f64,
    lon: f64,
) -> Result<Vec<Term<'a>>, rustler::Error> {
    let point = Point::new(lon, lat);

    tree.nearest_neighbors(&[lon, lat])
        .iter()
        .filter(|fg| fg.geom.contains(&point))
        .filter_map(|fg| {
            fg.feature.properties.as_ref().map(|props| {
                let pairs = props
                    .iter()
                    .map(|(s, v)| (s, v.as_str().unwrap_or("")))
                    .collect::<Vec<_>>();

                Term::map_from_pairs(env, &pairs)
            })
        })
        .collect::<Result<Vec<_>, rustler::Error>>()
}

rustler::init!("Elixir.RStarGeoPointProperties", load = on_load);
