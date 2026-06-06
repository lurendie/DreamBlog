import {
	deleteUpyunFile,
	getUpyunContents,
	uploadUpyunFile
} from '@/api/pictureHosting'

export function getBucketContents(bucket, path) {
	return getUpyunContents(path).then(res => res.data)
}

export function delFile(bucket, path) {
	return deleteUpyunFile(path).then(res => res.data)
}

export function upload(bucket, path, fileName, data) {
	return uploadUpyunFile(path, fileName, data).then(res => res.data)
}
