import posts from './_posts/index.js';
import { Request } from 'polka';
import { ServerResponse as Response } from 'http';

const contents = JSON.stringify(posts as Post[]);

export function get(_req: Request, res: Response) {
	res.writeHead(200, {
		'Content-Type': 'application/json'
	});

	res.end(contents);
}