/**
 * 任务队列，按固定时间间隔依次执行队列中的函数
 *
 * 例: taskQueue(()=>{console.log(123)},1000)
 * 例: const stop = taskQueue(fn, 1000); 组件卸载时调用 stop() 清理定时器
 *
 * 语义说明：所有任务（含第一个）都间隔 timeout 毫秒执行，首个任务不会立即执行。
 * 这样可保证相邻两个任务的执行时间至少相差 timeout，用于规避连续上传带来的 commit 版本号冲突（如 GitHub 409）。
 * 若需要首个任务立即执行，可改为手动先调用一次 fn()。
 */
let queue = []
let timer = null

function process() {
	if (queue.length === 0) {
		clearInterval(timer)
		timer = null
		return
	}
	let fn = queue.shift()
	fn()
	if (queue.length === 0) {
		clearInterval(timer)
		timer = null
	}
}

/**
 * 清空尚未执行的任务，并停止定时器（组件卸载时调用，避免因队列残留导致的副作用）
 */
export function clearTaskQueue() {
	queue = []
	if (timer) {
		clearInterval(timer)
		timer = null
	}
}

/**
 * 停止定时器但保留未执行的任务（若后续需要可自行处理 pending 任务）
 */
export function stopTaskQueue() {
	if (timer) {
		clearInterval(timer)
		timer = null
	}
}

/**
 * 把一个任务加入队列。若队列尚未启动，则启动定时器按 interval 间隔依次执行。
 * 返回一个 stop 函数，可在组件卸载时调用以停止调度。
 */
export function taskQueue(fn, timeout = 1000) {
	queue.push(fn)
	if (!timer) {
		//首个任务不立即执行，统一等一个 interval 后再开始，保证相邻任务间隔稳定
		timer = setInterval(process, timeout)
	}
	return stopTaskQueue
}