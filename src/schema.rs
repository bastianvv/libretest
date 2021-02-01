table! {
    requirements (id) {
        id -> Int4,
        creation_date -> Timestamp,
        code -> Nullable<Varchar>,
        status -> Int4,
        importance -> Nullable<Int4>,
        nature -> Nullable<Int4>,
        title -> Varchar,
        description -> Nullable<Varchar>,
    }
}
