table! {
    posts {
        id -> Integer,
        title -> VarChar,
        date -> Timestamp,
        html -> VarChar,
        content -> VarChar,
        summary -> VarChar,
        url -> VarChar,
        slug -> VarChar,
    }
}

table! {
    post_content(docid) {
        docid -> Integer,
        title -> VarChar,
        content -> VarChar,
    }
}
