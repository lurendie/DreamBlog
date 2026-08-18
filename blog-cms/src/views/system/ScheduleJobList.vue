<template>
	<div>
		<el-form inline>
			<el-form-item>
				<el-button type="primary" size="mini" icon="el-icon-plus" @click="addDialogVisible=true">添加</el-button>
			</el-form-item>
			<el-form-item>
				<el-button type="warning" size="mini" icon="el-icon-document-checked" @click="goLogPage">日志</el-button>
			</el-form-item>
		</el-form>

		<el-table :data="jobList">
			<el-table-column label="任务ID" prop="jobId" width="80"></el-table-column>
			<el-table-column label="Bean" prop="beanName"></el-table-column>
			<el-table-column label="方法名" prop="methodName"></el-table-column>
			<el-table-column label="参数" prop="params"></el-table-column>
			<el-table-column label="Cron" prop="cron"></el-table-column>
			<el-table-column label="状态" width="80">
				<template v-slot="scope">
					<el-switch v-model="scope.row.status" @change="jobStatusChanged(scope.row)"></el-switch>
				</template>
			</el-table-column>
			<el-table-column label="备注" prop="remark"></el-table-column>
			<el-table-column label="创建时间" width="170">
				<template v-slot="scope">{{ dateFormat(scope.row.createTime) }}</template>
			</el-table-column>
			<el-table-column label="操作" width="290">
				<template v-slot="scope">
					<el-button type="warning" icon="el-icon-caret-right" size="mini" @click="runOnce(scope.row.jobId)">执行一次</el-button>
					<el-button type="primary" icon="el-icon-edit" size="mini" @click="showEditDialog(scope.row)">编辑</el-button>
					<el-popconfirm title="确定删除吗？" icon="el-icon-delete" icon-color="red" @confirm="deleteJobById(scope.row.jobId)">
						<template #reference><el-button size="mini" type="danger" icon="el-icon-delete">删除</el-button></template>
					</el-popconfirm>
				</template>
			</el-table-column>
		</el-table>

		<!--分页-->
		<el-pagination @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="queryInfo.pageNum"
		               :page-sizes="[10, 20, 30, 50]" :page-size="queryInfo.pageSize" :total="total"
		               layout="total, sizes, prev, pager, next, jumper" background>
		</el-pagination>

		<!--添加任务对话框-->
		<el-dialog title="添加任务" width="50%" v-model="addDialogVisible" :close-on-click-modal="false" @close="addDialogClosed">
			<!--内容主体-->
			<el-form :model="addForm" :rules="formRules" ref="addFormRef" label-width="80px">
				<el-form-item label="类型" prop="beanName">
					<el-select v-model="addForm.beanName" placeholder="请选择任务类型" style="width: 100%">
						<el-option label="本地任务(local)" value="local"></el-option>
						<el-option label="HTTP(http)" value="http"></el-option>
						<el-option label="HTTP GET(http:get)" value="http:get"></el-option>
						<el-option label="HTTP POST(http:post)" value="http:post"></el-option>
						<el-option label="Shell(shell)" value="shell"></el-option>
					</el-select>
				</el-form-item>
				<el-form-item label="目标" prop="methodName">
					<el-input v-model="addForm.methodName" :placeholder="methodPlaceholder(addForm.beanName)"></el-input>
				</el-form-item>
				<el-form-item label="参数" prop="params">
					<el-input v-model="addForm.params" :placeholder="paramsPlaceholder(addForm.beanName)"></el-input>
				</el-form-item>
				<el-form-item label="cron" prop="cron">
					<el-input v-model="addForm.cron" placeholder="例如: */5 * * * * 或 */10 * * * * *"></el-input>
				</el-form-item>
				<el-form-item label="备注" prop="remark">
					<el-input v-model="addForm.remark"></el-input>
				</el-form-item>
			</el-form>
			<!--底部-->
			<template #footer>
				<el-button @click="addDialogVisible=false">取 消</el-button>
				<el-button type="primary" @click="addJob">确 定</el-button>
			</template>
		</el-dialog>

		<!--编辑任务对话框-->
		<el-dialog title="编辑任务" width="50%" v-model="editDialogVisible" :close-on-click-modal="false" @close="editDialogClosed">
			<!--内容主体-->
			<el-form :model="editForm" :rules="formRules" ref="editFormRef" label-width="80px">
				<el-form-item label="类型" prop="beanName">
					<el-select v-model="editForm.beanName" placeholder="请选择任务类型" style="width: 100%">
						<el-option label="本地任务(local)" value="local"></el-option>
						<el-option label="HTTP(http)" value="http"></el-option>
						<el-option label="HTTP GET(http:get)" value="http:get"></el-option>
						<el-option label="HTTP POST(http:post)" value="http:post"></el-option>
						<el-option label="Shell(shell)" value="shell"></el-option>
					</el-select>
				</el-form-item>
				<el-form-item label="目标" prop="methodName">
					<el-input v-model="editForm.methodName" :placeholder="methodPlaceholder(editForm.beanName)"></el-input>
				</el-form-item>
				<el-form-item label="参数" prop="params">
					<el-input v-model="editForm.params" :placeholder="paramsPlaceholder(editForm.beanName)"></el-input>
				</el-form-item>
				<el-form-item label="Cron" prop="cron">
					<el-input v-model="editForm.cron" placeholder="例如: */5 * * * * 或 */10 * * * * *"></el-input>
				</el-form-item>
				<el-form-item label="备注" prop="remark">
					<el-input v-model="editForm.remark"></el-input>
				</el-form-item>
			</el-form>
			<!--底部-->
			<template #footer>
				<el-button @click="editDialogVisible=false">取 消</el-button>
				<el-button type="primary" @click="editJob">确 定</el-button>
			</template>
		</el-dialog>
	</div>
</template>

<script>
	import Breadcrumb from "@/components/Breadcrumb";
	import {getJobList, updateJobStatus, runJobOnce, deleteJobById, addJob, editJob} from "@/api/schedule";

	export default {
		name: "ScheduleJobList",
		components: {Breadcrumb},
		data() {
			return {
				queryInfo: {
					pageNum: 1,
					pageSize: 10
				},
				jobList: [],
				total: 0,
				addDialogVisible: false,
				editDialogVisible: false,
				addForm: {
					beanName: 'local',
					methodName: '',
					params: '',
					cron: '',
					remark: ''
				},
				editForm: {},
				formRules: {
					beanName: [{required: true, message: '请选择任务类型', trigger: 'change'}],
					methodName: [{required: true, message: '请输入目标', trigger: 'blur'}],
					cron: [{required: true, message: '请输入cron表达式', trigger: 'blur'}],
				}
			}
		},
		created() {
			this.getData()
		},
		methods: {
			getData() {
				getJobList(this.queryInfo).then(res => {
					this.jobList = res.data.list
					this.total = res.data.total
				})
			},
			handleSizeChange(newSize) {
				this.queryInfo.pageSize = newSize
				this.getData()
			},
			handleCurrentChange(newPage) {
				this.queryInfo.pageNum = newPage
				this.getData()
			},
			jobStatusChanged(row) {
				let text = row.status ? '启用' : '暂停'
				this.$confirm(`确认要<strong style="color: red">${text}</strong>该任务吗?`, '提示', {
					confirmButtonText: '确定',
					cancelButtonText: '取消',
					type: 'warning',
					dangerouslyUseHTMLString: true
				}).then(() => {
					return updateJobStatus(row.jobId, row.status)
				}).then(res => {
					this.msgSuccess(res.msg)
				}).catch(err => {
					//取消或失败都回滚开关状态；仅请求真正失败时提示
					row.status = !row.status
					if (err !== 'cancel' && err !== 'close') {
						this.msgError((err && err.msg) || '操作失败')
					}
				})
			},
			runOnce(jobId) {
				this.$confirm('确认要立即执行一次该任务吗?', '提示', {
					confirmButtonText: '确定',
					cancelButtonText: '取消',
					type: 'warning'
				}).then(() => {
					return runJobOnce(jobId)
				}).then(res => {
					this.msgSuccess(res.msg)
				}).catch(err => {
					//取消/关闭对话框不提示；仅请求真正失败时提示
					if (err !== 'cancel' && err !== 'close') {
						this.msgError((err && err.msg) || '执行失败')
					}
				})
			},
			deleteJobById(jobId) {
				deleteJobById(jobId).then(res => {
					this.msgSuccess(res.msg)
					this.getData()
				})
			},
			showEditDialog(row) {
				this.editForm = {...row}
				this.editDialogVisible = true
			},
			addDialogClosed() {
				this.$refs.addFormRef.resetFields()
			},
			editDialogClosed() {
				this.editForm = {}
				this.$refs.editFormRef.resetFields()
			},
			addJob() {
				this.$refs.addFormRef.validate(valid => {
					if (valid) {
						addJob(this.addForm).then(res => {
							this.msgSuccess(res.msg)
							this.addDialogVisible = false
							this.getData()
						})
					}
				})
			},
			editJob() {
				this.$refs.editFormRef.validate(valid => {
					if (valid) {
						editJob(this.editForm).then(res => {
							this.msgSuccess(res.msg)
							this.editDialogVisible = false
							this.getData()
						})
					}
				})
			},
			goLogPage() {
				this.$router.push('/log/job')
			},
			methodPlaceholder(beanName) {
				switch (beanName) {
					case 'local':
						return '例如: cache.clear_all';
					case 'shell':
						return '例如: echo';
					case 'http':
					case 'http:get':
					case 'http:post':
						return '例如: https://example.com/api';
					default:
						return '请输入目标';
				}
			},
			paramsPlaceholder(beanName) {
				switch (beanName) {
					case 'local':
						return '本地任务一般无需参数';
					case 'shell':
						return '例如: hello world';
					case 'http':
					case 'http:get':
					case 'http:post':
						return 'JSON字符串或留空';
					default:
						return '请输入参数';
				}
			}
		}
	}
</script>

<style scoped>
	.el-button + span {
		margin-left: 10px;
	}

	.el-form--inline .el-form-item {
		margin-bottom: 0;
	}
</style>


