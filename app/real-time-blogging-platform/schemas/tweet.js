export default {
  name: 'tweet',
  title: 'Tweet',
  type: 'document',
  fields: [
    {
      name: 'text',
      title: 'Text you write in tweet',
      type: 'string',
    },
    {
      name: 'blockTweet',
      title: 'Block Tweet',
      description: 'Admin Controls ',
      type: 'boolean',
    },
    {
      name: 'username',
      title: 'Username',
      type: 'string',
    },
    {
      name: 'profile_img',
      title: 'Profile image',
      type: 'string',
    },
    {
      name: 'image',
      title: 'Tweet Image',
      type: 'string'
    },
    // {
    //   name: 'categories',
    //   title: 'Categories',
    //   type: 'array',
    //   of: [{type: 'reference', to: {type: 'category'}}],
    // },
    // {
    //   name: 'publishedAt',
    //   title: 'Published at',
    //   type: 'datetime',
    // },
    // {
    //   name: 'body',
    //   title: 'Body',
    //   type: 'blockContent',
    // },
  ],

  // preview: {
  //   select: {
  //     title: 'title',
  //     author: 'author.name',
  //     media: 'mainImage',
  //   },
  //   prepare(selection) {
  //     const {author} = selection
  //     return Object.assign({}, selection, {
  //       subtitle: author && `by ${author}`,
  //     })
  //   },
  // },
}
