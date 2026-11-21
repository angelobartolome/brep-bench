//! A cube of side 10 from the origin, the edge where its faces x = 10 and y = 10 meet chamfered by
//! a distance growing from 1 to 3 along it on the face x = 10 and by 2 on the face y = 10.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no chamfer whose distance varies along the edge: chamfer_two_edges and chamfer_distances each hold every distance constant along an edge".into())
}
