import {
	deleteGithubFile,
	getGithubContents,
	getGithubRepos,
	getGithubUser,
	uploadGithubFile
} from '@/api/pictureHosting'

//注意：name 参数为保留参数，当前实现未使用（仓库名 repos 已包含所需信息），
//保留签名以避免破坏调用方（如 GithubManage.vue 传入 name 的调用方式）。

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
