#![allow(unused_imports)]
#![allow(dead_code)]
use std::fs::File;
use std::io::{self, Read};
use std::mem::replace;
use std::path::{Path, PathBuf};

use crate::errors::*;
use crate::from_filename;
use crate::iter;
use crate::Finder;
use crate::Iter;

#[derive(Default, Clone)]
enum FileSourceType<'a> {
    #[default]
    Default,
    Filename(&'a Path),
    Path(&'a Path),
}

#[derive(Default, Clone)]
pub struct FileSource<'a>(FileSourceType<'a>);
pub struct ReadSource<'a>(&'a mut dyn Read);

trait DotenvIter<'a, I> {
    fn load(self) -> Result<Option<PathBuf>>;
    fn iter(self) -> Result<Option<Iter<I>>>;
}

#[derive(Default, Clone)]
pub struct DotenvBuilder<S> {
    source: S,
    optional: bool,
    overryde: bool,
}

impl<'a> DotenvBuilder<FileSource<'a>> {
    fn find_iter(&mut self) -> Result<(Option<PathBuf>, Option<Iter<File>>)> {
        let find_result = match self.source.0 {
            FileSourceType::Default => Finder::new().find(),
            FileSourceType::Filename(f) => Finder::new().filename(f).find(),
            FileSourceType::Path(p) => match File::open(p) {
                Err(e) => Err(Error::Io(e)),
                Ok(f) => {
                    let i = Iter::new(f);
                    let mut pb = PathBuf::new();
                    pb.push(p);
                    Ok((pb, i))
                }
            },
        };

        match find_result {
            Err(e) => {
                if self.optional && e.not_found() {
                    Ok((None, None))
                } else {
                    Err(e)
                }
            }
            Ok((pb, i)) => Ok((Some(pb), Some(i))),
        }
    }
}

impl<'a> DotenvIter<'a, File> for DotenvBuilder<FileSource<'a>> {
    fn load(mut self) -> Result<Option<PathBuf>> {
        let (pb, iter) = self.find_iter()?;

        if let Some(iter) = iter {
            if self.overryde {
                iter.load_override()?;
            } else {
                iter.load()?;
            }
        }

        Ok(pb)
    }

    fn iter(mut self) -> Result<Option<Iter<File>>> {
        let (_, iter) = self.find_iter()?;

        Ok(iter)
    }
}

impl<'a> DotenvBuilder<ReadSource<'a>> {}

impl<'a> DotenvIter<'a, &'a mut dyn Read> for DotenvBuilder<ReadSource<'a>> {
    fn iter(self) -> Result<Option<Iter<&'a mut dyn Read>>> {
        Ok(Some(Iter::new(self.source.0)))
    }

    fn load(self) -> Result<Option<PathBuf>> {
        let iter = Iter::new(self.source.0);
        if self.overryde {
            iter.load_override()?;
        } else {
            iter.load()?;
        }

        Ok(None)
    }
}

impl<S> DotenvBuilder<S> {
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn overryde(mut self) -> Self {
        self.overryde = true;
        self
    }
}
