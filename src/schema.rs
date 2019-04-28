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

allow_tables_to_appear_in_same_query!(posts, post_content);
