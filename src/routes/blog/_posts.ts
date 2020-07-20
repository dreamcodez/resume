// Ordinarily, you'd generate this data from markdown files in your
// repo, or fetch them from a database of some kind. But in order to
// avoid unnecessary dependencies in the starter template, and in the
// service of obviousness, we're just going to leave it here.

// This file is called `_posts.js` rather than `posts.js`, because
// we don't want to create an `/blog/posts` route — the leading
// underscore tells Sapper not to do that.


export interface Post {
	title: string,
	slug: string,
	html: string,
}

const posts: Post[] = [
	{
		title: 'So I Built a New Home Page.',
		slug: 'built-a-new-homepage',
		html: `
		    <i>July 19, 2020</i>
			<p>I used <a href='https://svelte.dev'>Svelte</a> to redo / re-imagine my home page.</p>

			<p>So far, it's a breeze and the framework mostly gets out of the way ;)</p>
		`
	},

];

posts.forEach(post => {
	post.html = post.html.replace(/^\t{3}/gm, '');
});

export default posts;
