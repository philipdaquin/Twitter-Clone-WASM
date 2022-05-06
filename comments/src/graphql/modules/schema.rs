use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::NaiveDateTime;
use super::resolver;
use crate::graphql::config::get_conn_from_ctx;


pub struct User { 
    pub id: ID
}
#[Object(extends)]
impl User { 
    #[graphql(external)]
    pub async fn id(&self, id: ID) -> User { 
        User { id }
    }
    #[graphql(name = "getAllUserComments")]
    pub async fn comments(&self, ctx: &Context<'_>, user_id: ID) -> Vec<CommentType> { 
        resolver::get_comments_by_user(parse_id(user_id),&get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect()
    }
    #[graphql(name = "getUserCommentFromPost")]
    pub async fn get_user_comment_post(&self, ctx: &Context<'_>, post_id: ID, user_id: ID) -> FieldResult<Vec<CommentType>> { 
        let user_comments = resolver::get_comment_of_user(parse_id(post_id), parse_id(user_id), &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect();
        Ok(user_comments)
    }
}

pub struct PostObject { 
    pub id: ID
}
#[Object(extends)]
impl PostObject { 
    #[graphql(external)]
    pub async fn id(&self, id: ID) -> PostObject { 
        PostObject { id }
    }
    #[graphql(name = "getAllCommentsFromPost")]
    pub async fn comments(&self, ctx: &Context<'_>, post_id: ID) -> Vec<CommentType> { 
        resolver::get_comments_by_post(parse_id(post_id), &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect()
    }
}

#[derive(Default)]
pub struct CommentQuery;

#[derive(SimpleObject)]
pub struct CommentType { 
    pub id: ID,
    pub author_id: User,
    pub post_id: PostObject,
    pub body: String,
    pub media: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[Object]
impl CommentQuery {
    /// Insert all the keys referencing to Post and User 
    #[graphql(entity)]
    pub async fn get_user_details(&self, #[graphql(key)] id: ID) -> User { 
        User { id }
    }
    pub async fn get_post_details(&self, #[graphql(key)] id: ID) -> PostObject { 
        PostObject { id }
    }
    #[graphql(name = "getAllComments")]
    pub async fn get_all(&self, ctx: &Context<'_>) -> Option<Vec<CommentType>> { 
        let comments = resolver::get_all_comments(&get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect();
        Some(comments)
    }
    #[graphql(name = "getCommentsFromPost")]
    pub async fn get_comments_from_post(&self, ctx: &Context<'_>, post_id: ID) -> Vec<CommentType> { 
        resolver::get_comments_by_post(parse_id(post_id), &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect()
    }
    #[graphql(name = "getComment")]
    pub async fn get_comment(&self, ctx: &Context<'_>, comment_id: ID) -> Option<CommentType> { 
        resolver::get_comments_by_id(parse_id(comment_id), &get_conn_from_ctx(ctx))
            .ok()
            .map(|f| CommentType::from(&f))
    }

}



fn parse_id(id: ID) -> i32 { 
    id.parse::<i32>().expect("")
}