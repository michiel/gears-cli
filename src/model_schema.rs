use juniper::FieldResult;
use juniper::RootNode;

use gears::structure::model::ModelDocument;

#[derive(GraphQLObject)]
#[graphql(description = "Page info")]
pub struct PageInfo {
    #[graphql(name = "startCursor")]
    pub start_cursor: Option<String>,
    #[graphql(name = "endCursor")]
    pub end_cursor: Option<String>,
    #[graphql(name = "hasNextPage")]
    pub has_next_page: bool,
}

const DEFAULT_PAGE_SIZE: i32 = 20;

#[derive(GraphQLInputObject)]
pub struct PagingParams {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

impl PagingParams {
    pub fn get_limit(&self) -> i32 {
        match self.limit {
            None => DEFAULT_PAGE_SIZE,
            Some(limit) => limit,
        }
    }

    pub fn get_cursor(&self) -> i64 {
        match self.cursor {
            None => 0,
            Some(ref cursor) => cursor.parse::<i64>().unwrap_or(0),
        }
    }
}

impl Default for PagingParams {
    fn default() -> Self {
        PagingParams {
            limit: Some(DEFAULT_PAGE_SIZE),
            cursor: None,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct ModelsFilterParams {
    pub uuid: Option<String>,
    pub name: Option<String>,
}

impl Default for ModelsFilterParams {
    fn default() -> Self {
        ModelsFilterParams {
            uuid: None,
            name: None,
        }
    }
}

/*
graphql_object!(ModelDocument: () |&self| {
    field id() -> &str {
        self.id.as_str()
    }

    field name() -> &str {
        self.name.as_str()
    }
});
*/

#[derive(GraphQLObject)]
#[graphql(description = "Connection")]
pub struct ModelConnection {
    //#[graphql(description = "This contains the Model results")]
    pub edges: Vec<Human>,
    #[graphql(name = "pageInfo")]
    pub page_info: PageInfo,
    pub cursor: Option<String>,
}

#[derive(GraphQLEnum)]
enum Status {
    Active,
    InActive,
    Disabled,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature")]
pub struct Human {
    id: String,
    name: String,
    appears_in: Vec<Status>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature")]
struct NewHuman {
    name: String,
    appears_in: Vec<Status>,
    home_planet: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field models(&executor,
                filter: Option<ModelsFilterParams>,
                paging: Option<PagingParams>
               ) -> FieldResult<ModelConnection> {

        // let conn = executor.context().db_pool.get()?;
        let filter = filter.unwrap_or_default();
        let paging = paging.unwrap_or_default();

        // let res = db_find_users(&conn, &filter, &paging)?;

        Ok(
            ModelConnection {
                // edges: res.items,
                edges: vec![
                    Human{
                        id: "1234".to_owned(),
                        name: "Johnny Cash".to_owned(),
                        appears_in: vec![Status::Disabled],
                        home_planet: "Earth".to_owned(),
                    }
                ],
                page_info: PageInfo {
                    start_cursor: None,
                    end_cursor: None,
                    has_next_page: false // res.has_more,
                },
                cursor: None // res.cursor,
            }
        )
    }

    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Johnny Cash".to_owned(),
            appears_in: vec![Status::Disabled],
            home_planet: "Earth".to_owned(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
