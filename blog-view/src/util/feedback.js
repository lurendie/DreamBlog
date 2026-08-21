let messagePromise
let notificationPromise

async function loadMessage() {
	if (!messagePromise) {
		messagePromise = Promise.all([
			import('element-plus/es/components/message/style/css'),
			import('element-plus/es/components/message/index.mjs')
		]).then(([, module]) => module.ElMessage)
	}
	return messagePromise
}

async function loadNotification() {
	if (!notificationPromise) {
		notificationPromise = Promise.all([
			import('element-plus/es/components/notification/style/css'),
			import('element-plus/es/components/notification/index.mjs')
		]).then(([, module]) => module.ElNotification)
	}
	return notificationPromise
}

export function showMessage(type, message) {
	loadMessage().then(ElMessage => {
		ElMessage[type](message)
	})
}

export function showNotification(options) {
	loadNotification().then(ElNotification => {
		ElNotification(options)
	})
}

export const lazyMessage = {
	success(message) {
		showMessage('success', message)
	},
	error(message) {
		showMessage('error', message)
	},
	info(message) {
		showMessage('info', message)
	},
	warning(message) {
		showMessage('warning', message)
	}
}

export function lazyNotify(options) {
	showNotification(options)
}
