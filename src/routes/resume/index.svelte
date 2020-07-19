<script context="module" type="ts">
	export function preload({ params, query }) {
		return this.fetch(`resume.json`).then(r => r.json()).then(resume => {
			return resume;
		});
	}
</script>

<script>
	export let jobs;
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

<h1>Experience</h1>

<ul>
	{#each jobs as job}
		<li>{job.company}</li>
	{/each}
</ul>