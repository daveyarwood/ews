pub enum Query {
    Id(i64),
    SearchString(String)
}

impl Query {
    pub fn new(query: String) -> Query {
        match query.parse::<i64>() {
            Ok(id) => Query::Id(id),
            Err(_) => Query::SearchString(query)
        }
    }
}
