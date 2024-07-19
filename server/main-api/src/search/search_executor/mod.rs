use serde::Serialize;
use tracing::error;

use crate::limited::vec::LimitedVec;
use crate::search::search_executor::parser::ParsedQuery;
use crate::search::search_executor::query::MSHit;

use super::{Highlighting, Limits};

mod formatter;
mod lexer;
mod merger;
mod parser;
mod query;

#[derive(Serialize, Debug, Clone)]
pub struct ResultsSection {
    facet: String,
    entries: Vec<ResultEntry>,
    n_visible: usize,
    #[serde(rename = "estimatedTotalHits")]
    estimated_total_hits: usize,
}

#[derive(Serialize, Default, Debug, Clone)]
struct ResultEntry {
    #[serde(skip)]
    hit: MSHit,
    id: String,
    r#type: String,
    name: String,
    subtext: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtext_bold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parsed_id: Option<String>,
}
#[tracing::instrument]
pub async fn do_geoentry_search(
    q: String,
    highlighting: Highlighting,
    limits: Limits,
) -> LimitedVec<ResultsSection> {
    let parsed_input = ParsedQuery::from(q.as_str());

    match query::GeoEntryQuery::from((&parsed_input, &limits, &highlighting))
        .execute()
        .await
    {
        Ok(response) => {
            let (section_buildings, mut section_rooms) = merger::merge_search_results(
                &limits,
                response.results.first().unwrap(),
                response.results.get(1).unwrap(),
                response.results.get(2).unwrap(),
            );
            let visitor = formatter::RoomVisitor::from((parsed_input, highlighting));
            section_rooms
                .entries
                .iter_mut()
                .for_each(|r| visitor.visit(r));

            match section_buildings.n_visible {
                0 => LimitedVec(vec![section_rooms, section_buildings]),
                _ => LimitedVec(vec![section_buildings, section_rooms]),
            }
        }
        Err(e) => {
            // error should be serde_json::error
            error!("Error searching for results: {e:?}");
            LimitedVec(vec![])
        }
    }
}

#[cfg(test)]
mod test{

    use pretty_assertions::assert_eq;

    use super::*;

    #[derive(serde::Deserialize)]
    struct TestQuery {
        target:String,
        query:String  ,
        among:Option<usize>,
    }

    impl TestQuery {
        fn load_good() -> Vec<Self> {
            serde_yaml::from_str(include_str!("test-queries.good.yaml")).unwrap()
        }
        fn load_bad() -> Vec<Self> {
            serde_yaml::from_str(include_str!("test-queries.bad.yaml")).unwrap()
        }
    }
    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_good_queries() {
        let highlighting = Highlighting::default();
        for query in TestQuery::load_good(){
            let actual = do_geoentry_search(query.query, ).await.0;
        }
    }
}
