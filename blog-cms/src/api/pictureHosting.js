import request from '@/util/request'

function uploadForm(fields, file) {
	const form = new FormData()
	Object.keys(fields).forEach(key => form.append(key, fields[key]))
	form.append('file', file)
	return form
}

export function getConfigs() {
	return request({
		url: 'pictureHosting/configs',
		method: 'GET'
	})
}

export function getGithubUser(token) {
	return request({
		url: 'pictureHosting/github/user',
		method: 'POST',
		data: {token}
	})
}

export function saveGithubConfig(token) {
	return request({
		url: 'pictureHosting/config/github',
		method: 'POST',
		data: {token}
	})
}

export function saveUpyunConfig(data) {
	return request({
		url: 'pictureHosting/config/upyun',
		method: 'POST',
		data
	})
}

export function saveTxyunConfig(data) {
	return request({
		url: 'pictureHosting/config/txyun',
		method: 'POST',
		data
	})
}

export function deleteConfig(provider) {
	return request({
		url: `pictureHosting/config/${provider}`,
		method: 'DELETE'
	})
}

export function getGithubRepos() {
	return request({
		url: 'pictureHosting/github/repos',
		method: 'GET'
	})
}

export function getGithubContents(repos, path) {
	return request({
		url: 'pictureHosting/github/contents',
		method: 'GET',
		params: {repos, path}
	})
}

export function deleteGithubFile(repos, path, sha) {
	return request({
		url: 'pictureHosting/github/file',
		method: 'DELETE',
		data: {repos, path, sha}
	})
}

export function uploadGithubFile(repos, path, fileName, file) {
	return request({
		url: 'pictureHosting/github/upload',
		method: 'POST',
		data: uploadForm({repos, path, fileName}, file),
		timeout: 30000
	})
}

export function getUpyunContents(path) {
	return request({
		url: 'pictureHosting/upyun/contents',
		method: 'GET',
		params: {path}
	})
}

export function deleteUpyunFile(path) {
	return request({
		url: 'pictureHosting/upyun/file',
		method: 'DELETE',
		data: {path}
	})
}

export function uploadUpyunFile(path, fileName, file) {
	return request({
		url: 'pictureHosting/upyun/upload',
		method: 'POST',
		data: uploadForm({path, fileName}, file),
		timeout: 30000
	})
}

export function getTxyunContents(path) {
	return request({
		url: 'pictureHosting/txyun/contents',
		method: 'GET',
		params: {path}
	})
}

export function deleteTxyunFile(path) {
	return request({
		url: 'pictureHosting/txyun/file',
		method: 'DELETE',
		data: {path}
	})
}

export function uploadTxyunFile(path, fileName, file) {
	return request({
		url: 'pictureHosting/txyun/upload',
		method: 'POST',
		data: uploadForm({path, fileName}, file),
		timeout: 30000
	})
}
