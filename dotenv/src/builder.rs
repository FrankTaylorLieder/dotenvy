use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::errors::*;
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
pub struct FileSource<'a> {
    source: FileSourceType<'a>,
    allow_missing: bool,
}

pub struct ReadSource<'a> {
    reader: &'a mut dyn Read,
}

pub trait BuilderFinalizer<'a, I> {
    fn load(self) -> Result<Option<PathBuf>>;
    fn iter(self) -> Result<Option<Iter<I>>>;
}

#[derive(Default, Clone)]
pub struct Builder<S> {
    source: S,
    override_duplicates: bool,
}

impl<'a> Builder<FileSource<'a>> {
    pub fn allow_missing(mut self) -> Self {
        self.source.allow_missing = true;
        self
    }

    fn find_iter(&mut self) -> Result<(Option<PathBuf>, Option<Iter<File>>)> {
        let find_result = match self.source.source {
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
                if self.source.allow_missing && e.not_found() {
                    Ok((None, None))
                } else {
                    Err(e)
                }
            }
            Ok((pb, i)) => Ok((Some(pb), Some(i))),
        }
    }
}

impl<'a> BuilderFinalizer<'a, File> for Builder<FileSource<'a>> {
    fn load(mut self) -> Result<Option<PathBuf>> {
        let (pb, iter) = self.find_iter()?;

        if let Some(iter) = iter {
            if self.override_duplicates {
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

impl<'a> Builder<ReadSource<'a>> {}

impl<'a> BuilderFinalizer<'a, &'a mut dyn Read> for Builder<ReadSource<'a>> {
    fn iter(self) -> Result<Option<Iter<&'a mut dyn Read>>> {
        Ok(Some(Iter::new(self.source.reader)))
    }

    fn load(self) -> Result<Option<PathBuf>> {
        let iter = Iter::new(self.source.reader);
        if self.override_duplicates {
            iter.load_override()?;
        } else {
            iter.load()?;
        }

        Ok(None)
    }
}

impl<S> Builder<S> {
    pub fn override_duplicates(mut self) -> Self {
        self.override_duplicates = true;
        self
    }
}

pub fn dotenv<'a>() -> Builder<FileSource<'a>> {
    Builder {
        source: FileSource {
            source: FileSourceType::Default,

            ..Default::default()
        },

        ..Default::default()
    }
}

pub fn from_filename<'a, P>(filename: &'a P) -> Builder<FileSource<'a>>
where
    P: AsRef<Path> + ?Sized,
{
    Builder {
        source: FileSource {
            source: FileSourceType::Filename(filename.as_ref()),

            ..Default::default()
        },

        ..Default::default()
    }
}

pub fn from_path<'a, P>(path: &'a P) -> Builder<FileSource<'a>>
where
    P: AsRef<Path> + ?Sized,
{
    Builder {
        source: FileSource {
            source: FileSourceType::Path(path.as_ref()),

            ..Default::default()
        },

        ..Default::default()
    }
}

pub fn from_read<'a, R>(reader: &'a mut R) -> Builder<ReadSource<'a>>
where
    R: Read,
{
    Builder {
        source: ReadSource { reader },
        override_duplicates: false,
    }
}
