<template>
	<div class="blog-list-page">
		<PageHeader
			eyebrow="内容管理"
			title="文章管理"
			description="筛选、编辑、置顶、推荐和可见性控制都在这里完成。"
		>
			<template #actions>
				<el-button @click="search">刷新</el-button>
				<el-button type="primary" @click="$router.push('/blog/write')">写文章</el-button>
			</template>
		</PageHeader>

		<el-card class="search-card">
			<div class="search-bar">
				<el-select v-model="queryInfo.categoryId" clearable placeholder="全部分类" @change="search">
					<el-option v-for="item in categoryList" :key="item.id" :label="item.name" :value="item.id" />
				</el-select>
				<el-input
					v-model="queryInfo.title"
					clearable
					placeholder="搜索标题"
					@clear="search"
					@keyup.enter="search"
				/>
				<el-button type="primary" @click="search">搜索</el-button>
			</div>
		</el-card>

		<el-card>
			<el-table :data="blogList" :empty-text="'暂无文章'">
				<el-table-column label="序号" type="index" width="70" />
				<el-table-column label="标题" prop="title" min-width="260" show-overflow-tooltip />
				<el-table-column label="分类" prop="category.name" width="140" />
				<el-table-column label="状态" width="120">
					<template #default="scope">
						<el-tag :type="visibilityType(scope.row)">
							{{ visibilityText(scope.row) }}
						</el-tag>
					</template>
				</el-table-column>
				<el-table-column label="置顶" width="90">
					<template #default="scope">
						<el-switch v-model="scope.row.top" @change="blogTopChanged(scope.row)" />
					</template>
				</el-table-column>
				<el-table-column label="推荐" width="90">
					<template #default="scope">
						<el-switch v-model="scope.row.recommend" @change="blogRecommendChanged(scope.row)" />
					</template>
				</el-table-column>
				<el-table-column label="创建时间" width="170">
					<template #default="scope">{{ dateFormat(scope.row.createTime) }}</template>
				</el-table-column>
				<el-table-column label="最近更新" width="170">
					<template #default="scope">{{ dateFormat(scope.row.updateTime) }}</template>
				</el-table-column>
				<el-table-column label="操作" width="220" fixed="right">
					<template #default="scope">
						<el-button text type="primary" @click="goBlogEditPage(scope.row.id)">编辑</el-button>
						<el-button text @click="editBlogVisibility(scope.row)">可见性</el-button>
						<el-popconfirm
							title="确定删除吗？"
							icon-color="red"
							@confirm="deleteBlogById(scope.row.id)"
						>
							<template #reference>
								<el-button text type="danger">删除</el-button>
							</template>
						</el-popconfirm>
					</template>
				</el-table-column>
			</el-table>

			<el-pagination
				:current-page="queryInfo.pageNum"
				:page-sizes="[10, 20, 30, 50]"
				:page-size="queryInfo.pageSize"
				:total="total"
				background
				layout="total, sizes, prev, pager, next, jumper"
				@size-change="handleSizeChange"
				@current-change="handleCurrentChange"
			/>
		</el-card>

		<!--编辑可见性状态对话框-->
		<el-dialog title="博客可见性" width="30%" v-model="dialogVisible">
			<!--内容主体-->
			<el-form label-width="50px" @submit.prevent>
				<el-form-item>
					<el-radio-group v-model="radio">
						<el-radio :label="1">公开</el-radio>
						<el-radio :label="2">私密</el-radio>
						<el-radio :label="3">密码保护</el-radio>
					</el-radio-group>
				</el-form-item>
				<el-form-item label="密码" v-if="radio===3">
					<el-input v-model="visForm.password"></el-input>
				</el-form-item>
				<el-form-item v-if="radio!==2">
					<el-row>
						<el-col :span="6">
							<el-switch v-model="visForm.appreciation" active-text="赞赏"></el-switch>
						</el-col>
						<el-col :span="6">
							<el-switch v-model="visForm.recommend" active-text="推荐"></el-switch>
						</el-col>
						<el-col :span="6">
							<el-switch v-model="visForm.commentEnabled" active-text="评论"></el-switch>
						</el-col>
						<el-col :span="6">
							<el-switch v-model="visForm.top" active-text="置顶"></el-switch>
						</el-col>
					</el-row>
				</el-form-item>
			</el-form>
			<!--底部-->
			<template #footer>
				<el-button @click="dialogVisible=false">取 消</el-button>
				<el-button type="primary" @click="saveVisibility">保存</el-button>
			</template>
		</el-dialog>
	</div>
</template>

<script>
	import PageHeader from '@/components/PageHeader'
	import {getDataByQuery, deleteBlogById, updateTop, updateRecommend, updateVisibility} from '@/api/blog'

	export default {
		name: "BlogList",
		components: {PageHeader},
		data() {
			return {
				queryInfo: {
					title: '',
					categoryId: null,
					pageNum: 1,
					pageSize: 10
				},
				blogList: [],
				categoryList: [],
				total: 0,
				dialogVisible: false,
				blogId: 0,
				radio: 1,
				visForm: {
					appreciation: false,
					recommend: false,
					commentEnabled: false,
					top: false,
					published: false,
					password: '',
				}
			}
		},
		created() {
			this.getData()
		},
		methods: {
			getData() {
				getDataByQuery(this.queryInfo).then(res => {
					this.blogList = res.data.blogs.list
					this.categoryList = res.data.categories
					this.total = res.data.blogs.total
				})
			},
			search() {
				this.queryInfo.pageNum = 1
				this.queryInfo.pageSize = 10
				this.getData()
			},
			//切换博客置顶状态
			blogTopChanged(row) {
				updateTop(row.id, row.top).then(res => {
					this.msgSuccess(res.msg);
				})
			},
			//切换博客推荐状态
			blogRecommendChanged(row) {
				updateRecommend(row.id, row.recommend).then(res => {
					this.msgSuccess(res.msg);
				})
			},
			//编辑博客可见性
			editBlogVisibility(row) {
				this.visForm = {
					appreciation: row.appreciation,
					recommend: row.recommend,
					commentEnabled: row.commentEnabled,
					top: row.top,
					published: row.published,
					password: row.password,
				}
				this.blogId = row.id
				this.radio = this.visForm.published ? (this.visForm.password !== '' ? 3 : 1) : 2
				this.dialogVisible = true
			},
			//修改博客可见性
			saveVisibility() {
				if (this.radio === 3 && (this.visForm.password === '' || this.visForm.password === null)) {
					return this.msgError("密码保护模式必须填写密码！")
				}
				if (this.radio === 2) {
					this.visForm.appreciation = false
					this.visForm.recommend = false
					this.visForm.commentEnabled = false
					this.visForm.top = false
					this.visForm.published = false
				} else {
					this.visForm.published = true
				}
				if (this.radio !== 3) {
					this.visForm.password = ''
				}
				updateVisibility(this.blogId, this.visForm).then(res => {
					this.msgSuccess(res.msg)
					this.getData()
					this.dialogVisible = false
				})
			},
			//监听 pageSize 改变事件
			handleSizeChange(newSize) {
				this.queryInfo.pageSize = newSize
				this.getData()
			},
			//监听页码改变的事件
			handleCurrentChange(newPage) {
				this.queryInfo.pageNum = newPage
				this.getData()
			},
			goBlogEditPage(id) {
				this.$router.push(`/blog/edit/${id}`)
			},
			deleteBlogById(id) {
				this.$confirm('此操作将永久删除该博客<strong style="color: red">及其所有评论</strong>，是否删除?<br>建议将博客置为<strong style="color: red">私密</strong>状态！', '提示', {
					confirmButtonText: '确定',
					cancelButtonText: '取消',
					type: 'warning',
					dangerouslyUseHTMLString: true
				}).then(() => {
					return deleteBlogById(id)
				}).then(res => {
					this.msgSuccess(res.msg)
					this.getData()
				}).catch(err => {
					//取消/关闭对话框不提示；仅请求真正失败时提示
					if (err !== 'cancel' && err !== 'close') {
						this.msgError((err && err.msg) || '删除失败')
					}
				})
			},
			visibilityText(row) {
				return row.published ? (row.password !== '' ? '密码保护' : '公开') : '私密'
			},
			visibilityType(row) {
				if (!row.published) return 'info'
				return row.password !== '' ? 'warning' : 'success'
			}
		}
	}
</script>

<style scoped>
	.blog-list-page {
		max-width: 1480px;
		margin: 0 auto;
	}

	.search-card {
		margin-bottom: 16px;
	}

	.search-bar {
		display: grid;
		grid-template-columns: 220px 1fr auto;
		gap: 12px;
	}

	.search-bar :deep(.el-select),
	.search-bar :deep(.el-input) {
		width: 100%;
	}

	:deep(.el-pagination) {
		margin-top: 18px;
		justify-content: flex-end;
	}

	:deep(.el-table .cell) {
		padding-top: 10px;
		padding-bottom: 10px;
	}

	@media screen and (max-width: 768px) {
		.search-bar {
			grid-template-columns: 1fr;
		}
	}
</style>

