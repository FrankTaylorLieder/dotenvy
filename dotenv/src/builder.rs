#![allow(unused_imports)]
#![allow(dead_code)]
use std::fs::File;
use std::io::{self, Read};
use std::mem::replace;
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
    Read(&'a mut dyn io::Read),
    Consumed,
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

enum ConcreteIter<'a> {
    File(Iter<File>),
    Read(Iter<&'a mut dyn io::Read>),
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

    pub fn from_read<R>(&mut self, reader: &'a mut R) -> &mut DotenvBuilder<'a>
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

    fn find_iter(&mut self) -> Result<(Option<PathBuf>, Option<ConcreteIter>)> {
        let source = replace(&mut self.source, Source::Consumed);
        let find_result = match source {
            Source::Default => match Finder::new().find() {
                Err(e) => Err(e),
                Ok((pb, i)) => Ok((Some(pb), ConcreteIter::File(i))),
            },
            Source::Filename(f) => match Finder::new().filename(f).find() {
                Err(e) => Err(e),
                Ok((pb, i)) => Ok((Some(pb), ConcreteIter::File(i))),
            },
            Source::Path(p) => match File::open(p) {
                Err(e) => Err(Error::Io(e)),
                Ok(f) => {
                    let i = Iter::new(f);
                    let mut pb = PathBuf::new();
                    pb.push(p);
                    Ok((Some(pb), ConcreteIter::File(i)))
                }
            },
            Source::Read(r) => Ok((None, ConcreteIter::Read(Iter::new(r)))),
            Source::Consumed => Err(Error::State(String::from("Source already consumed"))),
        };

        match find_result {
            Err(e) => {
                if self.optional && e.not_found() {
                    Ok((None, None))
                } else {
                    Err(e)
                }
            }
            Ok((pb, i)) => Ok((pb, Some(i))),
        }
    }

    pub fn load(&mut self) -> Result<Option<PathBuf>> {
        let overryde = self.overryde;
        match self.find_iter()? {
            (_, None) => Ok(None),
            (pb, Some(iter)) => {
                match iter {
                    ConcreteIter::File(iter) => {
                        if overryde {
                            iter.load_override()?;
                        } else {
                            iter.load()?;
                        }
                    }
                    ConcreteIter::Read(iter) => {
                        if overryde {
                            iter.load_override()?;
                        } else {
                            iter.load()?;
                        }
                    }
                }
                Ok(pb)
            }
        }
    }

    // pub fn load(&mut self) -> Result<Option<PathBuf>> {
    //     match self.find_iter()? {
    //         (_, None) => Ok(None),
    //         (pb, Some(iter)) => {
    //             if self.overryde {
    //                 iter.load_override()?;
    //             } else {
    //                 iter.load()?;
    //             }
    //             Ok(pb)
    //         }
    //     }
    // }

    // pub fn iter<R: Read>(&mut self) -> Result<Option<iter::Iter<R>>> {
    //     if let Source::Read(reader) = self.source {
    //         todo!() // return Ok(Some(Iter::new(reader)));
    //     }
    //     // Ok(self.find_iter()?.1)
    //     match self.find_iter()?.1 {
    //         ConcreteIter::File(f) => Ok(Some(f)),
    //         ConcreteIter::Read(r) => todo!(), //Ok(Some(r)),
    //     }
    // }
}
