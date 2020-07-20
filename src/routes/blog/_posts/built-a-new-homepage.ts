const markdownBody = `
I used [Svelte](https://svelte.dev) to re-imagine my home page.

So far, it's a breeze and the framework mostly gets out of the way ;)

The [marked npm package](https://www.npmjs.com/package/marked) is used to convert the body text of the posts to html of these blog posts.
`;

export default {
    title: 'So I Built a New Home Page',
    slug: 'built-a-new-homepage',
    date: '2020-07-19',
    markdownBody,
} as Post;