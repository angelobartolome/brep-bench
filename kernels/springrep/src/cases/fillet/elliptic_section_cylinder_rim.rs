//! A cylinder of radius 5 from z = 0 to 10, the circle where its side meets its top rounded by a
//! quarter ellipse reaching 1 across the top and 3 down the side.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no round whose section is a quarter ellipse, set back by a different distance on each face: its fillets are circular, and its only other section blend, the conic of fillet_conic, takes one setback for both faces and blends only a straight edge between planes".into())
}
