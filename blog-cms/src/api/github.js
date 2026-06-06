import {
	deleteGithubFile,
	getGithubContents,
	getGithubRepos,
	getGithubUser,
	uploadGithubFile
} from '@/api/pictureHosting'

export function getUserInfo(token) {
	return getGithubUser(token)
}

export function getUserRepos() {
	return getGithubRepos().then(res => res.data)
}

export function getReposContents(name, repos, path) {
	return getGithubContents(repos, path).then(res => res.data)
}

export function delFile(name, repos, filePath, data) {
	return deleteGithubFile(repos, filePath, data.sha).then(res => res.data)
}

export function upload(name, repos, path, fileName, data) {
	return uploadGithubFile(repos, path, fileName, data.file).then(res => res.data)
}
