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
	ul li {
		padding: 1em;
	}
	ul li:hover {
		background-color: #e9ffdc;
	}
	h4 {
		font-style: italic;
	}
	p.companyDescription {
		font-size: larger;
		font-family: 'helvetica';
	}
</style>

<svelte:head>
	<title>Matthew Elders' Resume</title>
</svelte:head>

<h1>Experience</h1>

<ul>
	{#each jobs as job}
		<li>
		    <h2><a href={job.companyWebsite} target="_blank">{job.company}</a></h2>
			<h3>{job.jobTitle}</h3>
			<h4>{job.start} to {job.end || 'Present'}</h4>
			<p class="companyDescription">{job.companyDescription}</p>
		</li>
	{/each}
</ul>