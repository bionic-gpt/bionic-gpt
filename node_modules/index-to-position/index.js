const getOffsets = ({
	oneBased,
	oneBasedLine = oneBased,
	oneBasedColumn = oneBased,
} = {}) => [oneBasedLine ? 1 : 0, oneBasedColumn ? 1 : 0];

// Performance https://github.com/sindresorhus/index-to-position/pull/9
function getPosition(text, textIndex, options) {
	const lineBreakBefore = textIndex === 0 ? -1 : text.lastIndexOf('\n', textIndex - 1);
	const [lineOffset, columnOffset] = getOffsets(options);
	return {
		line: lineBreakBefore === -1
			? lineOffset
			: text.slice(0, lineBreakBefore + 1).match(/\n/g).length + lineOffset,
		column: textIndex - lineBreakBefore - 1 + columnOffset,
	};
}

export default function indexToPosition(text, textIndex, options) {
	if (typeof text !== 'string') {
		throw new TypeError('Text parameter should be a string');
	}

	if (!Number.isInteger(textIndex)) {
		throw new TypeError('Index parameter should be an integer');
	}

	if (textIndex < 0 || textIndex > text.length) {
		throw new RangeError('Index out of bounds');
	}

	return getPosition(text, textIndex, options);
}
