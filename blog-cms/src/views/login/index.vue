<template>
	<div class="login-container">
		<div class="login-card">
			<div class="login-copy">
				<div class="login-brand">ZeroBlog Console</div>
				<div class="login-eyebrow">内容管理后台</div>
				<h1>后台管理</h1>
				<p>登录后管理文章、动态、友链、站点配置和统计数据。</p>
			</div>

			<el-form ref="loginForm" :model="loginForm" :rules="loginRules" class="login-form" auto-complete="on">
				<el-form-item prop="username">
					<el-input
						ref="username"
						v-model="loginForm.username"
						placeholder="用户名"
						name="username"
						type="text"
						tabindex="1"
						auto-complete="username"
						size="large"
					>
						<template #prefix>
							<svg-icon icon-class="user" />
						</template>
					</el-input>
				</el-form-item>

				<el-form-item prop="password">
					<el-input
						:key="passwordType"
						ref="password"
						v-model="loginForm.password"
						:type="passwordType"
						placeholder="密码"
						name="password"
						tabindex="2"
						auto-complete="current-password"
						size="large"
						@keyup.enter="handleLogin"
					>
						<template #prefix>
							<svg-icon icon-class="password" />
						</template>
						<template #suffix>
							<button type="button" class="pwd-toggle" @click="showPwd">
								<svg-icon :icon-class="passwordType === 'password' ? 'eye' : 'eye-open'" />
							</button>
						</template>
					</el-input>
				</el-form-item>

				<el-button class="login-button" :loading="loading" type="primary" @click.prevent="handleLogin">
					登录
				</el-button>
			</el-form>
		</div>
	</div>
</template>

<script>
	import {login} from "@/api/login";

	export default {
		name: 'Login',
		data() {
			return {
				loginForm: {
					username: '',
					password: ''
				},
				loginRules: {
					username: [
						{required: true, message: '请输入用户名', trigger: 'blur'},
					],
					password: [
						{required: true, message: '请输入密码', trigger: 'blur'},
					]
				},
				loading: false,
				passwordType: 'password',
			}
		},
		methods: {
			showPwd() {
				this.passwordType = this.passwordType === 'password' ? 'text' : 'password'
				this.$nextTick(() => {
					this.$refs.password.focus()
				})
			},
			handleLogin() {
				this.$refs.loginForm.validate(valid => {
					if (valid) {
						this.loading = true
						login(this.loginForm).then(res => {
							if (res.code === 200 && res.data && res.data.token) {
								this.msgSuccess(res.msg)
								window.localStorage.setItem('isLoggedIn', '1')
								window.localStorage.setItem('user', JSON.stringify(res.data.user))
								this.$router.push('/')
							} else {
								this.msgError((res && res.msg) || '登录失败')
							}
						}).catch(() => {
							this.msgError('登录失败，请重试')
						}).finally(() => {
							this.loading = false
						})
					}
				})
			}
		}
	}
</script>

<style scoped lang="scss">
	.login-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 24px;
		background: linear-gradient(180deg, #f8fafc 0%, #eef3f9 100%);
	}

	.login-card {
		width: min(980px, 100%);
		display: grid;
		grid-template-columns: minmax(0, 1.1fr) minmax(320px, 400px);
		align-items: stretch;
		overflow: hidden;
		border: 1px solid #dbe5f3;
		border-radius: 18px;
		background: #fff;
		box-shadow: 0 24px 60px rgba(30, 41, 59, 0.12);
	}

	.login-copy {
		display: flex;
		flex-direction: column;
		justify-content: center;
		padding: 56px 52px;
		background: linear-gradient(160deg, #172033 0%, #24324a 100%);
		color: #fff;
	}

	.login-brand {
		display: inline-flex;
		width: fit-content;
		margin-bottom: 18px;
		padding: 6px 10px;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.12);
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0;
	}

	.login-eyebrow {
		margin-bottom: 10px;
		color: rgba(255, 255, 255, 0.7);
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0;
	}

	.login-copy h1 {
		margin: 0;
		font-size: 36px;
		line-height: 1.2;
	}

	.login-copy p {
		max-width: 26rem;
		margin: 16px 0 0;
		color: rgba(255, 255, 255, 0.78);
		font-size: 15px;
		line-height: 1.8;
	}

	.login-form {
		padding: 56px 48px;
	}

	.login-form :deep(.el-form-item) {
		margin-bottom: 18px;
	}

	.login-form :deep(.el-input__wrapper) {
		box-shadow: 0 0 0 1px #d8e0ed inset !important;
		border-radius: 10px;
	}

	.login-form :deep(.el-input__wrapper.is-focus) {
		box-shadow: 0 0 0 1px #2563eb inset !important;
	}

	.login-form :deep(.el-input__prefix) {
		color: #64748b;
	}

	.login-form :deep(.el-input__inner) {
		height: 46px;
	}

	.pwd-toggle {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		border: 0;
		background: transparent;
		color: #94a3b8;
		cursor: pointer;
	}

	.login-button {
		width: 100%;
		height: 46px;
		margin-top: 8px;
		border-radius: 10px;
		font-weight: 700;
	}

	@media screen and (max-width: 860px) {
		.login-card {
			grid-template-columns: 1fr;
		}

		.login-copy {
			padding: 32px 24px;
		}

		.login-form {
			padding: 28px 24px 32px;
		}
	}
</style>
