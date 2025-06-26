<script context="module" type="ts">
  export function preload({ params, query }) {
    return this.fetch(`resume.json`)
      .then((r) => r.json())
      .then((resume) => {
        return resume;
      });
  }
</script>

<script>
  export let jobs;
  export let skills;
  export let education;
</script>

<svelte:head>
  <title>Matthew Elders' Resume</title>
</svelte:head>

<p>
  This web-based resume is a work in progress. For the 'stable' version of my
  resume:
</p>

<ul>
  <li>
    <a href="https://github.com/dreamcodez/resume/blob/main/README.md"
      >Markdown (GitHub) version</a
    >
  </li>
  <li>
    <a href="https://github.com/dreamcodez/resume/raw/main/README.pdf"
      >PDF Version</a
    >
  </li>
</ul>

<h1>Experience</h1>

<ul class="experience">
  {#each jobs as job}
    <li>
      <h2>
        <a class="company" href={job.companyWebsite} target="_blank"
          >{job.company} 🔗
        </a>
      </h2>
      <h3>{job.jobTitle}</h3>
      <h4>{job.start} to {job.end || "Present"}</h4>
      <p class="companyDescription">{job.companyDescription}</p>
    </li>
  {/each}
</ul>

<h1>Skills</h1>

<h2>Expert</h2>
<ul>
  {#each skills.filter((s) => s.level === "Expert") as skill}
    <li>{skill.name}</li>
  {/each}
</ul>

<h2>Fluent</h2>
<ul>
  {#each skills.filter((s) => s.level === "Fluent") as skill}
    <li>{skill.name}</li>
  {/each}
</ul>

<h2>Amateur</h2>
<ul>
  {#each skills.filter((s) => s.level === "Amateur") as skill}
    <li>{skill.name}</li>
  {/each}
</ul>

<h2>Basics</h2>
<ul>
  {#each skills.filter((s) => s.level === "Basics") as skill}
    <li>{skill.name}</li>
  {/each}
</ul>

<h1>Education</h1>

<ul class="experience">
  {#each education as edu}
    <li>
      <h2>{edu.institution}</h2>
      <h3>{edu.degree} - {edu.field}</h3>
      <h4>{edu.start} to {edu.end}</h4>
      {#if edu.description}
        <p class="companyDescription">{edu.description}</p>
      {/if}
    </li>
  {/each}
</ul>

<style>
  a.company {
    text-decoration: none;
  }
  ul {
    line-height: 1.5;
  }
  ul.experience {
    padding: 1em 0em;
    margin: 0;
    list-style: none;
  }
  ul.experience li {
    margin: 0;
    padding: 1em;
  }
  ul li:hover {
    border-radius: 25px;
    background-color: #e9ffdc;
  }
  h4 {
    font-style: italic;
  }
  p.companyDescription {
    font-size: larger;
    font-family: "helvetica";
  }
</style>
