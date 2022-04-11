use async_graphql::*;
use async_graphql::Error;


use super::models::{Track, Author};
#[derive(Default)]
pub struct TrackQuery;

#[Object]
impl TrackQuery {     
    #[graphql(name = "tracksForHome")]
    /// Query to get tracks arrays for the homepage grid 
    async fn tracks_for_home(&self, ctx: &Context<'_>) -> Result<Vec<Track>, Error> { 
        let mut list_track: Vec<Track> = vec![];

        let author =  Author { 
            id: "author_1".into(),
            name: "Philip Daquin".to_string(),
            photo: "https://res.cloudinary.com/dety84pbu/image/upload/v1606816219/kitty-veyron-sm_mctf3c.jpg".to_string(),
        };
        let track = Track { 
            id: "track_01".into() ,
            title: "Astro Kitty, Space Explorer".to_string(), 
            author,
            thumbnail: "https://res.cloudinary.com/dety84pbu/image/upload/v1598465568/nebula_cat_djkt9r.jpg".to_string(), 
            length: 1210, 
            modules_count: 6 
        };
        list_track.push(track);    
        Ok(list_track)
    }
}
