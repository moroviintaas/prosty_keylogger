use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PathFragment{
    Raw(String),
    Env(String),
}

impl PathFragment{

    /*
    pub fn join_slice(s: &[PathFragment]) -> Result<PathBuf, anyhow::Error>{
        let p: PathBuf = PathBuf::from(s.iter().map(|f|{
            match f{
                PathFragment::Raw(r) => r,
                PathFragment::Env(v) => {
                    &env::var(v)
                }
            }
        }))?;
        return Ok(p);
    }

     */

    pub fn join_slice(slice: &[PathFragment]) -> Result<PathBuf, anyhow::Error>{
        let mut ret = String::new();
        for s in slice{
            match s{
                PathFragment::Raw(r) => ret.extend(r.chars()),
                PathFragment::Env(v) => {
                    let mut x = env::var(v)?.clone();

                    ret.extend(&mut x.chars())
                }
            }
        }
        Ok(PathBuf::from(ret))
    }
}

