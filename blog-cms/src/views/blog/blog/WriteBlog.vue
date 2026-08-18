<template>
	<div class="write-page">
		<PageHeader
			eyebrow="内容编辑"
			:title="isEditing ? '编辑文章' : '写文章'"
			description="先把正文写清楚，再处理分类、标签和发布状态。"
		>
			<template #actions>
				<el-button @click="$router.push('/blog/list')">返回列表</el-button>
				<el-button type="primary" @click="submit">保存文章</el-button>
			</template>
		</PageHeader>

		<el-form :model="form" :rules="formRules" ref="formRef" label-position="top" class="write-layout">
			<div class="write-main">
				<el-card class="editor-card">
					<el-form-item label="文章标题" prop="title">
						<el-input v-model="form.title" placeholder="请输入标题" size="large" />
					</el-form-item>
					<el-form-item label="文章首图 URL" prop="firstPicture">
						<el-input v-model="form.firstPicture" placeholder="文章首图，用于随机文章展示" />
					</el-form-item>
					<el-form-item label="文章描述" prop="description">
						<MdEditor v-model="form.description" class="description-editor" />
					</el-form-item>
					<el-form-item label="文章正文" prop="content">
						<MdEditor v-model="form.content" class="content-editor" />
					</el-form-item>
				</el-card>
			</div>

			<aside class="write-aside">
				<el-card class="side-card">
					<div class="side-card__title">发布设置</div>
					<el-form-item label="可见性">
						<el-radio-group v-model="radio" class="visibility-group">
							<el-radio-button :label="1">公开</el-radio-button>
							<el-radio-button :label="2">私密</el-radio-button>
							<el-radio-button :label="3">密码</el-radio-button>
						</el-radio-group>
					</el-form-item>
					<el-form-item label="访问密码" v-if="radio === 3">
						<el-input v-model="form.password" placeholder="请输入访问密码" show-password />
					</el-form-item>
					<div v-if="radio !== 2" class="switch-grid">
						<el-switch v-model="form.appreciation" active-text="赞赏" />
						<el-switch v-model="form.recommend" active-text="推荐" />
						<el-switch v-model="form.commentEnabled" active-text="评论" />
						<el-switch v-model="form.top" active-text="置顶" />
					</div>
				</el-card>

				<el-card class="side-card">
					<div class="side-card__title">分类标签</div>
					<el-form-item label="分类" prop="cate">
						<el-select
							v-model="form.cate"
							allow-create
							filterable
							placeholder="请选择分类（输入可添加）"
							style="width: 100%;"
						>
							<el-option v-for="item in categoryList" :key="item.id" :label="item.name" :value="item.id" />
						</el-select>
					</el-form-item>
					<el-form-item label="标签" prop="tagList">
						<el-select
							v-model="form.tagList"
							allow-create
							filterable
							multiple
							placeholder="请选择标签（输入可添加）"
							style="width: 100%;"
						>
							<el-option v-for="item in tagList" :key="item.id" :label="item.name" :value="item.id" />
						</el-select>
					</el-form-item>
				</el-card>

				<el-card class="side-card">
					<div class="side-card__title">阅读数据</div>
					<el-form-item label="字数" prop="words">
						<el-input v-model="form.words" placeholder="自动计算阅读时长" type="number" />
					</el-form-item>
					<el-form-item label="阅读时长（分钟）" prop="readTime">
						<el-input v-model="form.readTime" placeholder="默认按 200 字/分钟" type="number" />
					</el-form-item>
					<el-form-item label="浏览次数" prop="views">
						<el-input v-model="form.views" placeholder="默认为 0" type="number" />
					</el-form-item>
				</el-card>

				<el-button type="primary" class="submit-button" @click="submit">保存文章</el-button>
			</aside>
		</el-form>
	</div>
</template>

<script>
	import PageHeader from '@/components/PageHeader'
	import { getCategoryAndTag, saveBlog, getBlogById, updateBlog } from '@/api/blog'

	export default {
		name: "WriteBlog",
		components: { PageHeader },
		data() {
			return {
				categoryList: [],
				tagList: [],
				dialogVisible: false,
				radio: 1,
				form: {
					title: '',
					firstPicture: '',
					description: '',
					content: '',
					cate: null,
					tagList: [],
					words: 0,
					readTime: 0,
					views: 0,
					appreciation: false,
					recommend: false,
					commentEnabled: false,
					top: false,
					published: false,
					password: '',
				},
				formRules: {
					title: [{ required: true, message: '请输入标题', trigger: 'change' }],
					firstPicture: [{ required: true, message: '请输入首图链接', trigger: 'change' }],
					cate: [{ required: true, message: '请选择分类', trigger: 'change' }],
					tagList: [{ required: true, message: '请选择标签', trigger: 'change' }],
					words: [{ required: true, message: '请输入文章字数', trigger: 'change' }],
				},
			}
		},
		computed: {
			isEditing() {
				return !!this.$route.params.id
			}
		},
		watch: {
			'form.words'(newValue) {
				this.form.readTime = newValue ? Math.round(newValue / 200) : null
			},
		},
		created() {
			this.getData()
			if (this.$route.params.id) {
				this.getBlog(this.$route.params.id)
			}
		},
		methods: {
			getData() {
				getCategoryAndTag().then(res => {
					this.categoryList = res.data.categories
					this.tagList = res.data.tags
				})
			},
			getBlog(id) {
				getBlogById(id).then(res => {
					this.computeCategoryAndTag(res.data)
					this.form = res.data
					this.radio = this.form.published ? (this.form.password !== '' ? 3 : 1) : 2
				})
			},
			computeCategoryAndTag(blog) {
				blog.cate = blog.category.id
				blog.tagList = []
				blog.tags.forEach(item => {
					blog.tagList.push(item.id)
				})
			},
			submit() {
				if (this.radio === 3 && (this.form.password === '' || this.form.password === null)) {
					return this.msgError("密码保护模式必须填写密码！")
				}
				this.$refs.formRef.validate(valid => {
					if (valid) {
						// 确保字段为数字类型
						this.form.words = parseInt(this.form.words) || 0
						this.form.readTime = parseInt(this.form.readTime) || 0
						this.form.views = parseInt(this.form.views) || 0

						if (this.radio === 2) {
							this.form.appreciation = false
							this.form.recommend = false
							this.form.commentEnabled = false
							this.form.top = false
							this.form.published = false
						} else {
							this.form.published = true
						}
						if (this.radio !== 3) {
							this.form.password = ''
						}
						if (this.$route.params.id) {
							this.form.category = null
							this.form.tags = null
							updateBlog(this.form).then(res => {
								this.msgSuccess(res.msg)
								this.$router.push('/blog/list')
							})
						} else {
							saveBlog(this.form).then(res => {
								this.msgSuccess(res.msg)
								this.$router.push('/blog/list')
							})
						}
					} else {
						this.dialogVisible = false
						return this.msgError('请填写必要的表单项')
					}
				})
			}
		}
	}
</script>

<style scoped>
	.write-page {
		max-width: 1480px;
		margin: 0 auto;
	}

	.write-layout {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 360px;
		gap: 18px;
		align-items: start;
	}

	.write-main,
	.write-aside {
		min-width: 0;
	}

	.write-aside {
		position: sticky;
		top: 86px;
		display: flex;
		flex-direction: column;
		gap: 14px;
	}

	.editor-card :deep(.el-form-item:last-child) {
		margin-bottom: 0;
	}

	.side-card__title {
		margin-bottom: 16px;
		color: #172033;
		font-size: 15px;
		font-weight: 700;
	}

	.visibility-group {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		width: 100%;
	}

	.visibility-group :deep(.el-radio-button__inner) {
		width: 100%;
	}

	.switch-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 12px;
	}

	.submit-button {
		width: 100%;
		height: 44px;
	}

	.description-editor {
		min-height: 240px;
	}

	.content-editor {
		min-height: 620px;
	}

	@media screen and (max-width: 1180px) {
		.write-layout {
			grid-template-columns: 1fr;
		}

		.write-aside {
			position: static;
		}
	}
</style>


