<template>
	<div>
		<el-card>
			<template #header>
				<span>修改当前登录账户</span>
			</template>
			<el-form :model="account" label-width="50px">
				<el-form-item label="账号">
					<el-input v-model="account.username"></el-input>
				</el-form-item>
				<el-form-item label="密码">
					<el-input v-model="account.password"></el-input>
				</el-form-item>
				<el-popconfirm title="确定修改吗？" icon="el-icon-user-solid" icon-color="#409EFF" @confirm="save">
					<template #reference><el-button type="primary" size="medium" icon="el-icon-check" :disabled="!account.username || !account.password">确认修改</el-button></template>
				</el-popconfirm>
			</el-form>
		</el-card>
	</div>
</template>

<script>
import {changeAccount} from "@/api/account";
import {clearLoginState, getStoredUser} from "@/util/storage";

export default {
	name: "Setting",
	data() {
		return {
			user: {},
			account: {
				username: '',
				password: ''
			}
		}
	},
	created() {
		this.user = getStoredUser()
		if (!this.user) {
			clearLoginState()
			this.$router.push('/login')
			return
		}
		this.account.username = this.user.username
	},
	methods: {
		save() {
			changeAccount(this.account).then(res => {
				this.msgSuccess(res.msg)
				this.logout()
			})
		},
		logout() {
			clearLoginState()
			this.$router.push('/login')
		}
	}
}
</script>

<style scoped>
.el-card {
	width: 50%;
}
</style>


