<template>
	<div>
		<el-form :model="form" label-position="top">
			<el-form-item label="动态内容" prop="content">
				<div class="md-editor-panel moment-editor">
					<MdEditor v-model="form.content" />
				</div>
			</el-form-item>

			<el-form-item label="点赞数" prop="likes" style="width: 50%">
				<el-input v-model="form.likes" type="number" placeholder="可选，默认为 0"></el-input>
			</el-form-item>

			<el-form-item label="创建时间" prop="createTime">
				<el-date-picker v-model="form.createTime" type="datetime" placeholder="可选，默认此刻" :editable="false" value-format="YYYY-MM-DD HH:mm:ss"></el-date-picker>
			</el-form-item>

			<el-form-item style="text-align: right;">
				<el-button type="info" @click="submit(false)">仅自己可见</el-button>
				<el-button type="primary" @click="submit(true)">发布动态</el-button>
			</el-form-item>
		</el-form>
	</div>
</template>

<script>
	import Breadcrumb from "@/components/Breadcrumb";
	import {getMomentById, saveMoment, updateMoment} from "@/api/moment";

	export default {
		name: "WriteMoment",
		components: {Breadcrumb},
		data() {
			return {
				form: {
					content: '',
					createTime: null,
					likes: 0,
					published: false
				},
			}
		},
		created() {
			if (this.$route.params.id) {
				this.getMoment(this.$route.params.id)
			}
		},
		methods: {
			getMoment(id) {
				getMomentById(id).then(res => {
					//后端返回的 createTime 为 "YYYY-MM-DDTHH:mm:ss"，转成与 value-format 一致的格式，避免回显失败
					const createTime = res.data.createTime
					const normalized = typeof createTime === 'string' && createTime.includes('T')
						? createTime.replace('T', ' ')
						: createTime
					this.form = {...res.data, createTime: normalized}
				})
			},
			submit(published) {
				this.form.published = published
				if (this.$route.params.id) {
					updateMoment(this.form).then(res => {
						this.msgSuccess(res.msg)
						this.$router.push('/blog/moment/list')
					})
				} else {
					saveMoment(this.form).then(res => {
						this.msgSuccess(res.msg)
						this.$router.push('/blog/moment/list')
					})
				}
			}
		}
	}
</script>

<style scoped>
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

	.moment-editor {
		height: 460px;
	}

</style>

