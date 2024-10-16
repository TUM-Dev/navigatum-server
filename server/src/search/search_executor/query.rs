use crate::search::search_executor::parser::{Filter, ParsedQuery, TextToken};
use crate::search::{Highlighting, Limits};
use actix_web::dev::ResourcePath;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::search::{MultiSearchResponse, SearchQuery, Selectors};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize, Default, Clone)]
#[allow(dead_code)]
pub(super) struct MSHit {
    ms_id: String,
    pub(super) id: String,
    pub(super) name: String,
    pub(super) arch_name: Option<String>,
    pub(super) r#type: String,
    pub(super) type_common_name: String,
    pub(super) parent_building_names: Vec<String>,
    parent_keywords: Vec<String>,
    pub(super) campus: Option<String>,
    address: Option<String>,
    usage: Option<String>,
    rank: i32,
}
impl Debug for MSHit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MSHit")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

struct GeoEntryFilters {
    default: String,
    rooms: String,
    buildings: String,
}
impl Debug for GeoEntryFilters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut base = f.debug_struct("GeoEntryFilters");
        if !self.default.is_empty() {
            base.field("default", &self.default);
        }
        if !self.rooms.is_empty() {
            base.field("rooms", &self.rooms);
        }
        if !self.buildings.is_empty() {
            base.field("buildings", &self.buildings);
        }
        base.finish()
    }
}

impl From<&Filter> for GeoEntryFilters {
    fn from(filters: &Filter) -> Self {
        let ms_filter = filters.as_meilisearch_filters();
        let separator = if ms_filter.is_empty() { " " } else { " AND " };
        Self {
            default: ms_filter.clone(),
            buildings: format!("facet = \"building\"{separator}{ms_filter}"),
            rooms: format!("facet = \"room\"{separator}{ms_filter}"),
        }
    }
}

pub(super) struct GeoEntryQuery {
    client: Client,
    parsed_input: ParsedQuery,
    limits: Limits,
    highlighting: Highlighting,
    filters: GeoEntryFilters,
    sorting: Vec<String>,
}

impl Debug for GeoEntryQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut base = f.debug_struct("GeoEntryQuery");
        base.field("parsed_input", &self.parsed_input)
            .field("limits", &self.limits)
            .field("highlighting", &self.highlighting)
            .field("filters", &self.filters);
        if !self.sorting.is_empty() {
            base.field("sorting", &self.sorting);
        }
        base.finish()
    }
}

impl From<(&Client, &ParsedQuery, &Limits, &Highlighting)> for GeoEntryQuery {
    fn from(
        (client, parsed_input, limits, highlighting): (
            &Client,
            &ParsedQuery,
            &Limits,
            &Highlighting,
        ),
    ) -> Self {
        Self {
            client: client.clone(),
            parsed_input: parsed_input.clone(),
            limits: *limits,
            highlighting: highlighting.clone(),
            filters: GeoEntryFilters::from(&parsed_input.filters),
            sorting: parsed_input.sorting.as_meilisearch_sorting(),
        }
    }
}

impl GeoEntryQuery {
    #[tracing::instrument(ret(level = tracing::Level::TRACE))]
    pub async fn execute(self) -> Result<MultiSearchResponse<MSHit>, Error> {
        let q_default = self.prompt_for_querying();
        let entries = self.client.index("entries");

        // due to lifetime shenanigans this is added here (I can't make it move down to the other statements)
        // If you can make it, please propose a PR, I know that this is really hacky ^^
        let sorting = self
            .sorting
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        // Currently ranking is designed to put buildings at the top if they equally
        // match the term compared to a room. For this reason there is only a search
        // for all entries and only rooms, search matching (and relevant) buildings can be
        // expected to be at the top of the merged search. However sometimes a lot of
        // buildings will be hidden (e.g. building parts), so the extra room search ....
        self.client
            .multi_search()
            .with_search_query(
                self.merged_query(&entries, &q_default)
                    .with_sort(&sorting)
                    .build(),
            )
            .with_search_query(
                self.buildings_query(&entries, &q_default)
                    .with_sort(&sorting)
                    .build(),
            )
            .with_search_query(
                self.rooms_query(&entries, &self.prompt_for_querying_room())
                    .with_sort(&sorting)
                    .build(),
            )
            .execute::<MSHit>()
            .await
    }

    fn prompt_for_querying(&self) -> String {
        self.parsed_input
            .tokens
            .clone()
            .into_iter()
            .map(|s| match s {
                TextToken::Text(t) => t,
                TextToken::SplittableText((t1, t2)) => format!("{t1}{t2}"),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn prompt_for_querying_room(&self) -> String {
        self.parsed_input
            .tokens
            .clone()
            .into_iter()
            .map(|s| match s {
                TextToken::Text(t) => t,
                TextToken::SplittableText((t1, t2)) => format!("{t1} {t2} {t1}{t2}"),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn common_query<'b: 'a, 'a>(
        &'b self,
        entries: &'a Index,
    ) -> SearchQuery<'a, meilisearch_sdk::DefaultHttpClient> {
        let semantic_ratio = match std::env::var("MEILI_HYBRID_RATIO").ok() {
            None => None,
            Some(s) if s.trim().is_empty() => None,
            Some(s) => Some(s),
        }
        .unwrap_or("0.1".into())
        .parse()
        .unwrap_or(0.1);
        SearchQuery::new(entries)
            .with_facets(Selectors::Some(&["facet"]))
            .with_highlight_pre_tag(&self.highlighting.pre)
            .with_highlight_post_tag(&self.highlighting.post)
            .with_attributes_to_highlight(Selectors::Some(&["name"]))
            .with_hybrid("default", semantic_ratio)
            .build()
    }

    fn merged_query<'a>(
        &'a self,
        entries: &'a Index,
        query: &'a str,
    ) -> SearchQuery<'a, meilisearch_sdk::DefaultHttpClient> {
        let mut s = self
            .common_query(entries)
            .with_query(query)
            .with_limit(self.limits.total_count)
            .build();
        if !self.filters.default.is_empty() {
            s = s.with_filter(&self.filters.default).build();
        }
        s
    }

    fn buildings_query<'a>(
        &'a self,
        entries: &'a Index,
        query: &'a str,
    ) -> SearchQuery<'a, meilisearch_sdk::DefaultHttpClient> {
        let semantic_ratio = match std::env::var("MEILI_HYBRID_RATIO").ok() {
            None => None,
            Some(s) if s.trim().is_empty() => None,
            Some(s) => Some(s),
        }
        .unwrap_or("0.1".into())
        .parse()
        .unwrap_or(0.1);
        self.common_query(entries)
            .with_query(query)
            .with_limit(2 * self.limits.buildings_count) // we might do reordering later
            .with_filter(&self.filters.buildings)
            .with_hybrid("default", semantic_ratio)
            .build()
    }

    fn rooms_query<'a>(
        &'a self,
        entries: &'a Index,
        query: &'a str,
    ) -> SearchQuery<'a, meilisearch_sdk::DefaultHttpClient> {
        self.common_query(entries)
            .with_query(query)
            .with_limit(self.limits.rooms_count)
            .with_filter(&self.filters.rooms)
            .build()
    }
}
