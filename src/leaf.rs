use crate::conf::seed::*;
use rleaf::error::*;
use rleaf::leafconfig::*;
use rleaf::leafcore::*;
use std::io;

fn leaf_configure(leaf: &mut Leafcore, conf: &SeedConf) -> Result<(), LeafConfigError> {
    match &conf.installation.pkglisturl {
        Some(list) => leaf.set_str_conf(CleafStringConfig::PKGLISTURL, &list.as_str())?,
        None => {}
    };

    match &conf.installation.force {
        Some(f) => {
            leaf.set_bool_conf(CleafBoolConfig::FORCE, *f)?;
        }
        None => {}
    };

    leaf.set_str_conf(
        CleafStringConfig::ROOTDIR,
        format!("{}/mount/", &conf.workdir).as_str(),
    )?;
    leaf.set_bool_conf(CleafBoolConfig::NOASK, true)?;

    info!(
        "Installing system to {} using leaf...",
        leaf.get_str_conf(CleafStringConfig::ROOTDIR)?
    );

    Ok(())
}

fn leaf_install(leaf: &mut Leafcore, conf: &SeedConf) -> Result<(), LeafCoreError> {
    leaf.a_update()?;

    let packages: Vec<&str> = conf
        .installation
        .packages
        .iter()
        .map(|s| s.as_str())
        .collect();
    leaf.a_install(&packages)?;

    Ok(())
}

pub fn leaf_install_system(conf: &SeedConf) -> Result<(), io::Error> {
    let mut leaf = Leafcore::new();

    match leaf_configure(&mut leaf, conf) {
        Ok(()) => (),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Leaf config error: {}", e),
            ))
        }
    };

    match leaf_install(&mut leaf, conf) {
        Ok(()) => Ok(()),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Leaf error: {}", e),
            ))
        }
    }
}
