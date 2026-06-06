export function parseJson(value, fallback = null) {
	if (!value) {
		return fallback
	}
	try {
		return JSON.parse(value)
	} catch (e) {
		return fallback
	}
}

export function getStoredUser() {
	const user = parseJson(window.localStorage.getItem('user'), null)
	return user && typeof user === 'object' ? user : null
}

export function clearLoginState() {
	window.localStorage.removeItem('token')
	window.localStorage.removeItem('user')
}
