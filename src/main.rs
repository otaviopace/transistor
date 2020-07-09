use j4rs::{JvmBuilder,InvocationArg, MavenArtifact, errors::J4RsError};
use std::convert::TryFrom;

static PEER: &str = "datomic.Peer";
static URI: &str = "datomic:dev://localhost:4334/hello";

fn main() -> Result<(), J4RsError>{
    let jvm = JvmBuilder::new().build()?;
    jvm.deploy_artifact(&MavenArtifact::from("com.datomic:datomic-pro:1.0.6165"))?;

    let uri = InvocationArg::try_from(URI)?;

    let peer = jvm.create_instance(
        PEER,
        &Vec::new(),
    )?;
    
    jvm.invoke_static(
        PEER,
        "createDatabase",
        &[uri]
    )?;

    Ok(())
}