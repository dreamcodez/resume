import sirv from 'sirv';
import polka from 'polka';
import compression from 'compression';
import * as sapper from '@sapper/server';

const { PORT, NODE_ENV } = process.env;
const dev = NODE_ENV === 'development';

process.on('SIGINT', terminate.bind(0, 'SIGINT'));
process.on('SIGTERM', terminate.bind(0, 'SIGTERM'));

polka() // You can also use Express
	.use(
		compression({ threshold: 0 }),
		sirv('static', { dev }),
		sapper.middleware()
	)
	.listen(PORT, (err: Error) => {
		if (err) console.log('error', err);
	});

function terminate(signal: string) {
	console.log(`${signal} received; aborting process.`)
	process.exit(0);
}