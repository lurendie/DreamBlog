<template>
	<!--私密文章密码对话框-->
	<el-dialog title="请输入受保护文章密码" :width="dialogWidth" v-model="dialogVisible"
	           :lock-scroll="false" :before-close="blogPasswordDialogClosed">
		<!--内容主体-->
		<el-form :model="blogPasswordForm" :rules="formRules" ref="formRef" label-width="80px">
			<el-form-item label="密码" prop="password">
				<el-input v-model="blogPasswordForm.password"></el-input>
			</el-form-item>
		</el-form>
		<!--底部-->
		<template #footer>
			<el-button @click="blogPasswordDialogClosed">取 消</el-button>
			<el-button type="primary" @click="submitBlogPassword">确 定</el-button>
		</template>
	</el-dialog>
</template>

<script>
	import {mapState} from "vuex";
	import {SET_BLOG_PASSWORD_DIALOG_VISIBLE} from "../../store/mutations-types";
	import {checkBlogPassword} from "@/api/blog";
	import {setBlogVerified} from "@/util/storage";

	export default {
		name: "BlogPasswordDialog",
		computed: {
			...mapState(['blogPasswordForm']),
			dialogWidth() {
				return document.body.clientWidth <= 767 ? '92%' : '30%'
			},
			dialogVisible: {
				get() {
					return this.$store.state.blogPasswordDialogVisible
				},
				set(value) {
					this.$store.commit(SET_BLOG_PASSWORD_DIALOG_VISIBLE, value)
				}
			}
		},
		data() {
			return {
				formRules: {
					password: [{required: true, message: '请输入密码', trigger: 'change'}]
				}
			}
		},
		methods: {
			blogPasswordDialogClosed() {
				this.$refs.formRef.resetFields()
				this.$store.commit(SET_BLOG_PASSWORD_DIALOG_VISIBLE, false)
			},
			submitBlogPassword() {
				this.$refs.formRef.validate(valid => {
					if (valid) {
						checkBlogPassword(this.blogPasswordForm).then(res => {
							if (res.code === 200) {
								this.msgSuccess(res.msg)
								//res.data 可能返回 token 字符串，也可能返回整个文章对象；只在字符串时才把 token 存入 localStorage，
								//否则仅存"已验证"布尔标记，避免把大对象写入 localStorage
								setBlogVerified(this.blogPasswordForm.blogId, res.data)
								this.$router.push(`/blog/${this.blogPasswordForm.blogId}`)
								this.blogPasswordDialogClosed()
							} else {
								this.msgError(res.msg)
							}
						}).catch(() => {
							this.msgError("请求失败")
						})
					}
				})
			}
		}
	}
</script>

<style scoped>

</style>
