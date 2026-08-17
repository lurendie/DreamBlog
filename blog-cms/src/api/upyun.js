import {
	deleteUpyunFile,
	getUpyunContents,
	uploadUpyunFile
} from '@/api/pictureHosting'

//注意：bucket 参数为保留参数，当前实现未使用（后端从已配置的又拍云账号识别 bucket），
//保留签名以避免破坏调用方（如 UpyunManage.vue 传入 bucket 的调用方式）。

export function getBucketContents(bucket, path) {
	return getUpyunContents(path).then(res => res.data)
}

export function delFile(bucket, path) {
	return deleteUpyunFile(path).then(res => res.data)
}

export function upload(bucket, path, fileName, data) {
	return uploadUpyunFile(path, fileName, data).then(res => res.data)
}
