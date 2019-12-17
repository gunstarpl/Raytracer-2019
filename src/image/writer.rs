use crate::image::Image;
use std::path::{ Path, PathBuf };

#[derive(Debug)]
pub enum Error
{
    MissingInput,
    MissingOutput,
    InvalidPath,
    SaveFailed
}

pub trait Format
{
    fn save(&self, image: &Image, path: &Path) -> Result<(), Error>;
    fn get_name(&self) -> &'static str;
}

pub struct Writer<'a>
{
    format: Box<dyn Format + 'a>,
    image: Option<&'a Image>,
    path: Option<PathBuf>
}

impl<'a> Writer<'a>
{
    pub fn new<FormatType: Format + 'a>(format: FormatType) -> Writer<'a>
    {
        Writer
        {
            format: Box::new(format),
            image: None,
            path: None
        }
    }

    pub fn input(mut self, image: &'a Image) -> Self
    {
        self.image = Some(image);
        self
    }

    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self
    {
        self.path = Some(PathBuf::from(path.as_ref()));
        self
    }

    pub fn save(self) -> Result<(), Error>
    {
        let image = self.image.ok_or(Error::MissingInput)?;
        let path = self.path.as_ref().ok_or(Error::MissingOutput)?;

        {
            let begin_time = std::time::Instant::now();

            match path.parent()
            {
                None => return Err(Error::InvalidPath),
                Some(directory) =>
                {
                    let _ = std::fs::create_dir_all(directory);
                }
            };

            self.format.save(image, path)?;

            println!("Saved {} file in {} seconds.", self.format.get_name(), begin_time.elapsed().as_secs_f32());
        }

        Ok(())
    }
}
