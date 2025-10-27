import process from 'node:process';
import {Buffer} from 'node:buffer';

const hook = (stream, options, transform, sharedOutput) => {
	if (typeof options !== 'object') {
		transform = options;
		options = {};
	}

	options = {
		silent: true,
		once: false,
		...options,
	};

	let unhookFunction;
	const output = sharedOutput ?? [];

	const promise = new Promise(resolve => {
		const {write} = stream;

		const unhook = () => {
			stream.write = write;
			resolve();
		};

		stream.write = (output_, encoding, callback) => {
			const stringOutput = String(output_);
			output.push(stringOutput);

			let callbackReturnValue;
			if (transform) {
				callbackReturnValue = transform(stringOutput, unhook);
			}

			if (options.once) {
				unhook();
			}

			if (options.silent) {
				return typeof callbackReturnValue === 'boolean' ? callbackReturnValue : true;
			}

			let returnValue;
			if (typeof callbackReturnValue === 'string') {
				returnValue = typeof encoding === 'string' ? Buffer.from(callbackReturnValue).toString(encoding) : callbackReturnValue;
			}

			returnValue ||= (Buffer.isBuffer(callbackReturnValue) ? callbackReturnValue : output_);

			return write.call(stream, returnValue, encoding, callback);
		};

		unhookFunction = unhook;
	});

	promise.unhook = unhookFunction;

	// Add output getter that returns concatenated strings
	Object.defineProperty(promise, 'output', {
		get() {
			return output.join('');
		},
	});

	return promise;
};

export function hookStd(options, transform) {
	// Handle case where options is actually the transform function
	if (typeof options === 'function') {
		transform = options;
		options = {};
	}

	// Handle case where no arguments are provided
	options ||= {};

	const streams = options.streams ?? [process.stdout, process.stderr];

	// Use a shared output array for all streams to preserve interleaved order
	const sharedOutput = [];

	const streamPromises = streams.map(stream => hook(stream, options, transform, sharedOutput));

	const promise = Promise.all(streamPromises);
	promise.unhook = () => {
		for (const streamPromise of streamPromises) {
			streamPromise.unhook();
		}
	};

	// Add output getter that returns interleaved output from shared array
	Object.defineProperty(promise, 'output', {
		get() {
			return sharedOutput.join('');
		},
	});

	return promise;
}

export function hookStdout(...arguments_) {
	return hook(process.stdout, ...arguments_);
}

export function hookStderr(...arguments_) {
	return hook(process.stderr, ...arguments_);
}
