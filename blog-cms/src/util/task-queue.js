let queue = []
let running = false

const sleep = timeout => new Promise(resolve => setTimeout(resolve, timeout))

async function process() {
	if (running) return
	running = true

	while (queue.length) {
		const task = queue.shift()
		try {
			// 等待任务真正完成，避免多个 GitHub Contents 请求同时更新分支产生 409。
			await task.fn()
		} catch (error) {
			// 单个任务失败不能阻塞后续文件；任务本身负责通知上传组件失败。
			console.error('上传任务执行失败', error)
		}
		if (queue.length && task.timeout > 0) {
			await sleep(task.timeout)
		}
	}

	running = false
}

/**
 * 清空尚未执行的任务（组件卸载时调用，避免队列残留导致副作用）。
 */
export function clearTaskQueue() {
	queue = []
}

/**
 * 停止后续任务调度并清空等待中的任务。
 */
export function stopTaskQueue() {
	queue = []
}

/**
 * 将异步任务加入队列，前一个任务完成后才会执行下一个任务。
 */
export function taskQueue(fn, timeout = 1000) {
	queue.push({fn, timeout})
	void process()
	return stopTaskQueue
}
