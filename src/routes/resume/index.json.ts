import { default as jobs, Job } from './_jobs';
import { Request } from 'polka';
import { ServerResponse as Response } from 'http';

export interface ResumeContents {
	jobs: Job[],
}

const contents: string = JSON.stringify({
	jobs
} as ResumeContents);

export function get(_req: Request, res: Response) {
	res.writeHead(200, {
		'Content-Type': 'application/json'
	});

	res.end(contents);
}