function encodePathSegment(value) {
	return encodeURIComponent(value)
}

export function githubCdnUrl(login, repos, path) {
	const segments = [login, repos, ...String(path || '').split('/').filter(Boolean)]
	return `https://fastly.jsdelivr.net/gh/${segments.map(encodePathSegment).join('/')}`
}
