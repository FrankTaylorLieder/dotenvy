#![allow(unused_imports)]
#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use crate::errors::*;
use crate::from_filename;
use crate::iter;

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

struct Builder<'a> {
    source: Source<'a>,
    optional: bool,
    overryde: bool, // override is a keyword!
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self {
            source: Source::Default,
            optional: false,
            overryde: false,
        }
    }
}

impl<'a> Builder<'a> {
    fn from_filename<P>(&mut self, filename: &'a P) -> &mut Builder<'a>
    where
        P: AsRef<Path> + ?Sized,
    {
        self.source = Source::Filename(filename.as_ref());
        self
    }

    fn from_path<P>(&mut self, path: &'a P) -> &mut Builder<'a>
    where
        P: AsRef<Path> + ?Sized,
    {
        self.source = Source::Path(path.as_ref());
        self
    }

    fn from_read<R>(&mut self, reader: &'a R) -> &mut Builder<'a>
    where
        R: io::Read,
    {
        self.source = Source::Read(reader);
        self
    }

    fn optional(&mut self) -> &mut Builder<'a> {
        self.optional = true;
        self
    }

    fn overryde(&mut self) -> &mut Builder<'a> {
        self.overryde = true;
        self
    }

    fn load(&mut self) -> Result<PathBuf> {
        todo!()
    }

    fn iter(&mut self) -> Result<iter::Iter<File>> {
        todo!()
    }
}

fn run() {
    let mut b = Builder::default();
    let p = "hello";
    {
        b.from_filename(p);
    }

    b;
}

mod tests {
    use super::*;

    #[test]
    fn simple() -> Result<()> {
        Builder::default().optional().load()?;
        Ok(())
    }
}
