//! A half cylinder of radius 5 from z = 0 to 6 on the side x ≥ 0, the half circle where its side
//! meets its top chamfered by a distance growing linearly with the angle from 1 to 2.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no chamfer whose distance varies along the edge: chamfer_two_edges and chamfer_distances each hold every distance constant along an edge".into())
}
