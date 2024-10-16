use meilisearch_sdk::client::Client;
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
#[tracing::instrument(skip(client))]
pub async fn do_geoentry_search(
    client: &Client,
    q: String,
    highlighting: Highlighting,
    limits: Limits,
) -> LimitedVec<ResultsSection> {
    let parsed_input = ParsedQuery::from(q.as_str());

    match query::GeoEntryQuery::from((client, &parsed_input, &limits, &highlighting))
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
mod test {
    use super::*;
    use crate::setup::tests::MeiliSearchTestContainer;
    use std::fmt::{Display, Formatter};

    #[derive(serde::Deserialize)]
    struct TestQuery {
        target: String,
        query: String,
        among: Option<usize>,
        comment: Option<String>,
    }

    impl TestQuery {
        fn load_good() -> Vec<Self> {
            serde_yaml::from_str(include_str!("test-queries.good.yaml")).unwrap()
        }
        fn load_bad() -> Vec<Self> {
            serde_yaml::from_str(include_str!("test-queries.bad.yaml")).unwrap()
        }
        fn actual_matches_among(&self, actual: &[ResultsSection]) -> bool {
            let among = self.among.unwrap_or(1);
            let mut acceptable_range = actual.iter().flat_map(|r| r.entries.clone()).take(among);
            acceptable_range.any(|r| r.id == self.target)
        }
        async fn search(&self, client: &Client) -> Vec<ResultsSection> {
            do_geoentry_search(
                client,
                self.query.clone(),
                Highlighting::default(),
                Limits::default(),
            )
            .await
            .0
        }
    }
    impl Display for TestQuery {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "'{query}' should get '{target}' in {among} results",
                query = self.query,
                target = self.target,
                among = self.among.unwrap_or(1),
            )?;
            if let Some(comment) = &self.comment {
                write!(f, " # {comment}")?;
            }
            Ok(())
        }
    }
    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_good_queries() {
        let ms = MeiliSearchTestContainer::new(true).await;
        crate::setup::meilisearch::load_data(&ms.client, true)
            .await
            .unwrap();
        for query in TestQuery::load_good() {
            let actual = query.search(&ms.client).await;
            assert!(
                query.actual_matches_among(&actual),
                "{query}\n\
                Since it can't, please move it to .bad list"
            );

            // redacting estimatedTotalHits is because this can change quite easily and without a good reason
            insta::with_settings!({
                info => &format!("{query}"),
                description => query.comment.unwrap_or_default(),
            }, {
                insta::assert_yaml_snapshot!(actual, { ".**.estimatedTotalHits" => "[estimatedTotalHits]"});
            });
        }
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_bad_queries() {
        let ms = MeiliSearchTestContainer::new(true).await;
        crate::setup::meilisearch::load_data(&ms.client, true)
            .await
            .unwrap();
        for query in TestQuery::load_bad() {
            let actual = query.search(&ms.client).await;
            assert!(
                !query.actual_matches_among(&actual),
                "{query}\n\
                Since it can, please move it to .good list"
            );

            // redacting estimatedTotalHits is because this can change quite easily and without a good reason
            insta::with_settings!({
                info => &format!("{query}"),
                description => query.comment.unwrap_or_default(),
            }, {
                insta::assert_yaml_snapshot!(actual, { ".**.estimatedTotalHits" => "[estimatedTotalHits]"});
            });
        }
    }
}
