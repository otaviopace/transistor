use j4rs::{JvmBuilder,InvocationArg, MavenArtifact};
use std::convert::TryFrom;

static PEER: &str = "datomic.Peer";
static URI: &str = "datomic:dev://localhost:4334/hello";

fn main() {
    let jvm = JvmBuilder::new().build().unwrap();
    jvm.deploy_artifact(&MavenArtifact::from("com.datomic:datomic-pro:1.0.6165")).unwrap();

    let uri = InvocationArg::try_from(URI).unwrap();

    // jvm.static_class(PEER).unwrap();
    // jvm.invoke_static(
    //     PEER,
    //     "createDatabase",
    //     &[uri]
    // ).unwrap();


    let peer = jvm.create_instance(
        PEER,
        &Vec::new(),
    ).unwrap();

    let json_instance = jvm.invoke(
        &peer,
        "createDatabase",
        &[InvocationArg::try_from(URI).unwrap()], 
    ).unwrap();
}