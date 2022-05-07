// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import { sanityClient } from '../../sanity'
import { Tweet } from '../../typings'


type Data = {
  tweet: Tweet[]
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {



//   res.status(200).json({ name: 'John Doe' })
}

