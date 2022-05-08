// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import { groq } from 'next-sanity'
import {sanityClient} from '../../sanity'
import { Comment } from '../../typings'

const comment_query = groq`
  *[_type == "comment" && references(*[_type == "tweet" && _id == $tweetId]._id)] { 
  id,
 ...
  } | order(created_at desc)
`


type Data = Comment[];

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const { tweetId} = req.query;
  const comments: Comment[] = await sanityClient.fetch(comment_query, { 
    tweetId: tweetId
  })
  console.log('Comments >>> ', comments)


  res.status(200).json(comments)
}
