
use async_graphql::*;

#[derive(SimpleObject, Clone, PartialEq)]
#[graphql(name = "trackUsers")]
pub struct Track { 
    #[graphql(skip)]
    pub id: ID,
    /// The track's title 
    pub title: String,
    /// The track's main author 
    pub author: Author,
    /// The track's main illustration to display in track card or track page detail 
    pub thumbnail: String,
    /// The track's approximate length to complete, in minutes    
    pub length: i32,
    /// The number of modules this track contain
    pub modules_count: i32 
}

#[derive(SimpleObject, Clone, PartialEq)]
/// Author of a complete track 
pub struct Author { 
    pub id: ID,
    /// Author's first and last name 
    pub name: String,
    /// Author's profile picture url
    pub photo: String,
}



