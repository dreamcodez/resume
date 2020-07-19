<script context="module" type="ts">
	export function preload({ params, query }) {
		return this.fetch(`resume.json`).then(r => r.json()).then(jobs => {
			return { jobs };
		});
	}
</script>

<script type="ts">
	export let jobs: Job[];
</script>

<style>
	ul {
		margin: 0 0 1em 0;
		line-height: 1.5;
	}
</style>

<svelte:head>
	<title>Matthew Elders' Resume</title>
</svelte:head>

<h1>Recent posts</h1>

<ul>
	{#each posts as post}
		<!-- we're using the non-standard `rel=prefetch` attribute to
				tell Sapper to load the data for the page as soon as
				the user hovers over the link or taps it, instead of
				waiting for the 'click' event -->
		<li><a rel='prefetch' href='blog/{post.slug}'>{post.title}</a></li>
	{/each}
</ul>