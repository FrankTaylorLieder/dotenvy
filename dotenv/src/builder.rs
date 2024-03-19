#![allow(unused_imports)]
#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use crate::from_filename;
use crate::iter;
use crate::Finder;
use crate::{errors::*, Iter};

// Builder
//
// Options:
//   - Source, one of: default (.env), from_file(filename), from_path(path), from_reader(reader)
//   - optional()
//   - override()
//
// Finalisers:
//   - load() - load into env
//   - iter() - stream found env vars

enum Source<'a> {
    Default,
    Filename(&'a Path),
    Path(&'a Path),
    Read(&'a dyn io::Read),
}

pub struct DotenvBuilder<'a> {
    source: Source<'a>,
    optional: bool,
    overryde: bool, // override is a keyword!
}

impl<'a> Default for DotenvBuilder<'a> {
    fn default() -> Self {
        Self {
            source: Source::Default,
            optional: false,
            overryde: false,
        }
    }
}

pub fn build<'a>() -> DotenvBuilder<'a> {
    DotenvBuilder::default()
}

impl<'a> DotenvBuilder<'a> {
    pub fn from_filename<P>(&mut self, filename: &'a P) -> &mut DotenvBuilder<'a>
    where
        P: AsRef<Path> + ?Sized,
    {
        self.source = Source::Filename(filename.as_ref());
        self
    }

    pub fn from_path<P>(&mut self, path: &'a P) -> &mut DotenvBuilder<'a>
    where
        P: AsRef<Path> + ?Sized,
    {
        self.source = Source::Path(path.as_ref());
        self
    }

    pub fn from_read<R>(&mut self, reader: &'a R) -> &mut DotenvBuilder<'a>
    where
        R: io::Read,
    {
        self.source = Source::Read(reader);
        self
    }

    pub fn optional(&mut self) -> &mut DotenvBuilder<'a> {
        self.optional = true;
        self
    }

    pub fn overryde(&mut self) -> &mut DotenvBuilder<'a> {
        self.overryde = true;
        self
    }

    fn find_iter(&mut self) -> Result<(Option<PathBuf>, Option<Iter<File>>)> {
        let find_result = match self.source {
            Source::Default => Finder::new().find(),
            Source::Filename(f) => Finder::new().filename(f).find(),
            Source::Path(p) => match File::open(p) {
                Err(e) => Err(Error::Io(e)),
                Ok(f) => {
                    let iter = Iter::new(f);
                    let mut pb = PathBuf::new();
                    pb.push(p);
                    Ok((pb, iter))
                }
            },
            Source::Read(_r) => todo!(),
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

    pub fn load(&mut self) -> Result<Option<PathBuf>> {
        match self.find_iter()? {
            (_, None) => Ok(None),
            (pb, Some(iter)) => {
                if self.overryde {
                    iter.load()?;
                } else {
                    iter.load_override()?;
                }
                Ok(pb)
            }
        }
    }

    pub fn iter(&mut self) -> Result<Option<iter::Iter<File>>> {
        Ok(self.find_iter()?.1)
    }
}
