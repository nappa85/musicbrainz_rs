extern crate musicbrainz_rs;

use musicbrainz_rs::entity::series::*;
use musicbrainz_rs::Search;

#[test]
fn should_search_series() {
    let query = SeriesSearchQuery::query_builder()
        .series("now that's what i call music")
        .and()
        .comment("denmark")
        .build();

    let result = Series::search(query).execute().unwrap();

    assert!(result
        .entities
        .iter()
        .any(|series| series.id == "e38f1211-5aa0-4b91-9490-1c3d00e7ebca"));
}
