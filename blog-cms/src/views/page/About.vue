<template>
	<div>
		<el-form :model="form" :rules="formRules" ref="formRef" label-position="top">
			<el-form-item class="about-title-field" label="标题" prop="title">
				<el-input v-model="form.title" placeholder="请输入标题"></el-input>
			</el-form-item>

			<el-row class="about-meta-row" :gutter="20">
				<el-col :span="12">
					<el-form-item label="网易云歌曲ID" prop="musicId">
						<el-input v-model="form.musicId" type="number" placeholder="请输入网易云歌曲ID（可选）"></el-input>
					</el-form-item>
				</el-col>
				<el-col :span="12">
					<el-form-item label="评论开关">
						<el-switch v-model="form.commentEnabled" active-text="评论"></el-switch>
					</el-form-item>
				</el-col>
			</el-row>

			<el-form-item label="正文" prop="content">
				<div class="md-editor-panel about-editor">
					<MdEditor v-model="form.content" />
				</div>
			</el-form-item>

			<el-form-item style="text-align: right;">
				<el-button type="primary" icon="el-icon-check" @click="submit">保存</el-button>
			</el-form-item>
		</el-form>
	</div>
</template>

<script>
	import Breadcrumb from "@/components/Breadcrumb";
	import {getAbout, updateAbout} from "@/api/about";

	export default {
		name: "About",
		components: {Breadcrumb},
		data() {
			return {
				form: {
					title: '',
					musicId: null,
					content: '',
					commentEnabled: true
				},
				formRules: {
					title: [{required: true, message: '请输入标题', trigger: 'change'}],
				}
			}
		},
		created() {
			this.getData()
		},
		methods: {
			getData() {
				getAbout().then(res => {
					this.form.title = res.data.title
					this.form.musicId = res.data.musicId
					this.form.content = res.data.content
					this.form.commentEnabled = res.data.commentEnabled === 'true' ? true : false
				})
			},
			submit() {
				this.$refs.formRef.validate(valid => {
					if (valid) {
						// 允许留空；只有填写时才校验纯数字
						const reg = /^\d{1,}$/
						if (this.form.musicId !== null && this.form.musicId !== '' && !reg.test(this.form.musicId)) {
							return this.msgError("歌曲ID有误")
						}
						updateAbout(this.form).then(res => {
							this.msgSuccess(res.msg)
						})
					} else {
						return this.msgError('请填写必要的表单')
					}
				})
			}
		}
	}
</script>

<style scoped>
	.about-title-field,
	.about-meta-row {
		width: min(100%, 720px);
	}

	.md-editor-panel {
		width: 100%;
		overflow: hidden;
		border: 1px solid #e4e9f2;
		border-radius: 8px;
		background: #fff;
	}

	.md-editor-panel :deep(.md-editor) {
		height: 100% !important;
		border: 0 !important;
		border-radius: 0 !important;
	}

	.about-editor {
		height: 460px;
	}

	@media screen and (max-width: 768px) {
		.about-title-field,
		.about-meta-row {
			width: 100%;
		}

		.about-meta-row :deep(.el-col) {
			max-width: 100%;
			flex: 0 0 100%;
		}

		.about-editor {
			height: 360px;
		}
	}
</style>

