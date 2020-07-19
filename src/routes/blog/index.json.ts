import posts from './_posts.js';
import { Request} from 'polka';
import { ServerResponse as Response } from 'http';

const contents = JSON.stringify(posts.map(post => {
	return {
		title: post.title,
		slug: post.slug
	};
}));

export function get(req: Request, res: Response) {
	res.writeHead(200, {
		'Content-Type': 'application/json'
	});

	res.end(contents);
}