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
    optional: bool,
}
pub struct ReadSource<'a> {
    reader: &'a mut dyn Read,
}

pub trait BuilderFinalizer<'a, I> {
    fn load(self) -> Result<Option<PathBuf>>;
    fn iter(self) -> Result<Option<Iter<I>>>;
}

#[derive(Default, Clone)]
pub struct Builder2<S> {
    source: S,
    overryde: bool,
}

impl<'a> Builder2<FileSource<'a>> {
    pub fn optional(mut self) -> Self {
        self.source.optional = true;
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
                if self.source.optional && e.not_found() {
                    Ok((None, None))
                } else {
                    Err(e)
                }
            }
            Ok((pb, i)) => Ok((Some(pb), Some(i))),
        }
    }
}

impl<'a> BuilderFinalizer<'a, File> for Builder2<FileSource<'a>> {
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

impl<'a> Builder2<ReadSource<'a>> {}

impl<'a> BuilderFinalizer<'a, &'a mut dyn Read> for Builder2<ReadSource<'a>> {
    fn iter(self) -> Result<Option<Iter<&'a mut dyn Read>>> {
        Ok(Some(Iter::new(self.source.reader)))
    }

    fn load(self) -> Result<Option<PathBuf>> {
        let iter = Iter::new(self.source.reader);
        if self.overryde {
            iter.load_override()?;
        } else {
            iter.load()?;
        }

        Ok(None)
    }
}

impl<S> Builder2<S> {
    pub fn overryde(mut self) -> Self {
        self.overryde = true;
        self
    }
}

pub fn dotenv<'a>() -> Builder2<FileSource<'a>> {
    Builder2 {
        source: FileSource {
            source: FileSourceType::Default,
            optional: false,
        },

        ..Default::default()
    }
}

pub fn from_filename<'a, P>(filename: &'a P) -> Builder2<FileSource<'a>>
where
    P: AsRef<Path> + ?Sized,
{
    Builder2 {
        source: FileSource {
            source: FileSourceType::Filename(filename.as_ref()),
            optional: false,
        },

        ..Default::default()
    }
}

pub fn from_path<'a, P>(path: &'a P) -> Builder2<FileSource<'a>>
where
    P: AsRef<Path> + ?Sized,
{
    Builder2 {
        source: FileSource {
            source: FileSourceType::Path(path.as_ref()),
            optional: false,
        },

        ..Default::default()
    }
}

pub fn from_read<'a, R>(reader: &'a mut R) -> Builder2<ReadSource<'a>>
where
    R: Read,
{
    Builder2 {
        source: ReadSource { reader },
        overryde: false,
    }
}
