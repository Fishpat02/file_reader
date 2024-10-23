pub mod dbutils {
    use crate::schema::schema::*;
    use sqlx::{Pool, Postgres, QueryBuilder};

    const TABLE: &str = "files";

    pub async fn read_files_from_db(pool: &Pool<Postgres>) -> Result<Vec<DBFile>, sqlx::Error> {
        let files: Vec<DBFile> = sqlx::query_as(format!("SELECT * FROM {TABLE};").as_str())
            .fetch_all(pool)
            .await?;

        Ok(files)
    }

    pub async fn write_files_to_db(
        pool: &Pool<Postgres>,
        files: &Vec<File>,
    ) -> Result<(), sqlx::Error> {
        let db_files: Vec<File> = read_files_from_db(pool)
            .await?
            .into_iter()
            .map(|file| file.file)
            .collect();

        let files_to_push = if db_files.is_empty() {
            files.clone()
        } else {
            files
                .clone()
                .into_iter()
                .filter(|file| !db_files.contains(&file))
                .collect()
        };

        if files_to_push.is_empty() {
            ()
        } else {
            let mut query_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO files (filename,date_created) ");

            query_builder.push_values(files_to_push, |mut b, file| {
                b.push_bind(file.filename).push_bind(file.date_created);
            });

            query_builder.build().execute(pool).await?;
        }

        Ok(())
    }
}
