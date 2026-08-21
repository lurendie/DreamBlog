const TAG_COLOR_NAMES = new Set([
	'red',
	'orange',
	'yellow',
	'olive',
	'green',
	'teal',
	'blue',
	'violet',
	'purple',
	'pink',
	'brown',
	'grey',
	'black',
])

export function normalizeTagColor(color) {
	if (typeof color !== 'string') {
		return 'red'
	}
	const normalized = color.trim()
	return TAG_COLOR_NAMES.has(normalized) ? normalized : 'red'
}

export function tagColorClass(tagOrColor) {
	const color = typeof tagOrColor === 'string' ? tagOrColor : tagOrColor?.color
	return `me-${normalizeTagColor(color)}`
}
