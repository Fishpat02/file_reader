pub mod fsutils {
    use crate::schema::schema::*;
    use std::{fs, time::SystemTime};

    pub async fn read_files_from_fs(path: &str) -> Result<Vec<File>, std::io::Error> {
        let mut files: Vec<File> = vec![];

        let dir = fs::read_dir(path).expect("Could not read from directory");

        for file in dir {
            let current_file = &file?;

            let filename = current_file.file_name();
            let filename = filename.into_string().ok().unwrap();

            let date_created = current_file.metadata()?.created()?;
            let date_created = get_naive_time_from_systemtime(&date_created);

            files.push(File {
                filename,
                date_created,
            });
        }

        Ok(files)
    }

    fn get_naive_time_from_systemtime(time: &SystemTime) -> chrono::NaiveDate {
        use chrono::{DateTime, Utc};

        let utc_time: DateTime<Utc> = time.to_owned().into();
        utc_time.date_naive()
    }
}
