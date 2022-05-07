import { createClient} from 'next-sanity'

// lib/config.js
export const config = {
    dataset: process.env.NEXT_PUBLIC_SANITY_DATASET || 'production',
    projectId: process.env.NEXT_PUBLIC_SANITY_PROJECT_ID,
    apiVersion: '2021-10-21', // Learn more: https://www.sanity.io/docs/api-versioning
    
    useCdn: process.env.NODE_ENV === 'production',
  }

  // Set up the client for fetching data in the getProps page functions
export const sanityClient = createClient(config)
