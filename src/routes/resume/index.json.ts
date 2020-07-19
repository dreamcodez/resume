import { default as jobs, Job } from './_jobs';
import { Request } from 'polka';
import { ServerResponse as Response } from 'http';
import { default as skills, Skill } from './_skills';

export interface ResumeContents {
	jobs: Job[],
	skills: Skill[],
}

const contents: string = JSON.stringify({
	jobs,
	skills,
} as ResumeContents);

export function get(_req: Request, res: Response) {
	res.writeHead(200, {
		'Content-Type': 'application/json'
	});

	res.end(contents);
}