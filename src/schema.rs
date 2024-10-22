pub mod schema {
    use sqlx::prelude::FromRow;

    #[derive(PartialEq, FromRow, Debug, Clone)]
    #[allow(dead_code)]
    pub struct File {
        pub filename: String,
        pub date_created: chrono::NaiveDate,
    }

    #[derive(PartialEq, FromRow, Debug, Clone)]
    #[allow(dead_code)]
    pub struct DBFile {
        pub id: i32,
        #[sqlx(flatten)]
        pub file: File,
    }

    impl From<DBFile> for File {
        fn from(value: DBFile) -> Self {
            value.file
        }
    }
}
